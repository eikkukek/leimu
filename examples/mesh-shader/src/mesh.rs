use core::{
    hash::{self, Hash},
    cell::UnsafeCell,
};

use leimu::{
    EventResult, core::{
        collections::{AHashMap, AHashSet, VecDeque},
        slice,
    }, default,
    error::Context,
    mem::{
        reserve::{ReserveError, ReservePolicy},
        vec::{StdVecBase}
    }
};

use indexmap::{IndexMap, IndexSet};

use super::shader_types::*;

pub struct FixedPolicy;

unsafe impl ReservePolicy<u32> for FixedPolicy {

    #[inline]
    fn can_grow() -> bool {
        false
    }

    #[inline]
    fn grow(current: u32, required: usize) -> Result<u32, ReserveError<()>> {
        if required > current as usize {
            Err(ReserveError::max_capacity_exceeded(current, required, ()))
        } else {
            Ok(current)
        }
    }

    #[inline]
    fn grow_infallible(current: u32, required: usize) -> u32 {
        if required > current as usize {
            panic!("maximum capacity of {current} exceeded")
        }
        current
    }
}

pub type FixedBuffer<T> = StdVecBase<T, u32, FixedPolicy>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl Hash for Vertex {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        slice::value_as_bytes(self).hash(state);
    }
}

impl PartialEq for Vertex {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        slice::value_as_bytes(self) ==
        slice::value_as_bytes(other)
    }
}

impl Eq for Vertex {}

#[derive(Clone, Debug)]
pub struct Meshlet {
    local_vertices: FixedBuffer<u32>,
    hashed_vertices: AHashMap<u32, (u32, u32)>,
    local_triangles: FixedBuffer<[u32; 3]>,
    triangles_umapped: FixedBuffer<[u32; 3]>,
    origin_add: glam::Vec3,
    center_add: glam::Vec3,
    center: glam::Vec3,
    bounding_sphere_radius: f32,
}

impl Meshlet {

    fn new(
        max_local_vertices: u32,
        max_local_primitives: u32,
    ) -> Self {
        Self {
            local_vertices: FixedBuffer::with_capacity(max_local_vertices),
            hashed_vertices: AHashMap::new(),
            local_triangles: FixedBuffer::with_capacity(max_local_primitives),
            triangles_umapped: FixedBuffer::with_capacity(max_local_primitives),
            origin_add: default(),
            center_add: default(),
            center: default(),
            bounding_sphere_radius: 0.0,
        }
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.local_vertices.len() + 3 > self.local_vertices.capacity() ||
        self.local_triangles.len() == self.local_triangles.capacity()
    }

    #[inline]
    pub fn center(&self) -> glam::Vec3 {
        self.center
    }

    #[inline]
    pub fn origin(&self) -> glam::Vec3 {
        self.origin_add / 3.0
    }

    #[inline]
    pub fn bounding_sphere_radius(&self) -> f32 {
        self.bounding_sphere_radius
    }

    #[inline]
    pub fn local_vertices(&self) -> &[u32] {
        &self.local_vertices
    }

    #[inline]
    pub fn local_vertices_full_as_bytes(&self) -> &[u8] {
        slice::as_bytes(unsafe {
            slice::from_raw_parts(
                self.local_vertices.as_ptr(),
                self.local_vertices.capacity() as usize,
            )
        })
    }

    #[inline]
    pub fn local_triangles(&self) -> &[[u32; 3]] {
        &self.local_triangles
    }

    #[inline]
    pub fn local_triangles_full_as_bytes(&self) -> &[u8] {
        slice::as_bytes(unsafe {
            slice::from_raw_parts(
                self.local_triangles.as_ptr(),
                self.local_triangles.capacity() as usize,
           )
        })
    }

    #[inline]
    fn should_split(&self, other: &Self) -> bool {
        let threshold = self.local_triangles.capacity() / 4;
        if !(self.local_triangles.len() < threshold ||
            other.local_triangles.len() < threshold)
        {
            return false
        }
        if self.local_triangles.len() < other.local_triangles.len() {
            for idx in &self.local_vertices {
                if other.hashed_vertices.contains_key(idx) {
                    return true
                }
            }
        } else {
            for idx in &other.local_vertices {
                if self.hashed_vertices.contains_key(idx) {
                    return true
                }
            }
        }
        false
    }

    #[inline]
    fn split(self, other: Self, vertices: &[Vertex]) -> (Self, Self)
    {
        let threshold = self.local_triangles.capacity() / 4;
        let s = |mut s: Self, mut other: Self| -> (Self, Self) {
            while s.local_triangles.len() <= threshold &&
                s.local_vertices.len() + 2 < s.local_vertices.capacity()
            {
                let Some(take) = other.triangles_umapped
                    .iter()
                    .enumerate()
                    .filter(|(_, tri)| {
                        (**tri).into_iter().filter(|idx|
                            s.hashed_vertices.contains_key(idx)
                        ).count() > 1
                    }).max_by_key(|&(_, tri)| {
                        let bounds = s.bounding_sphere_radius();
                        let center = s.center();
                        let mut score = 0.0;
                        let mut new_vertices = 3;
                        for idx in tri {
                            if s.hashed_vertices
                                .contains_key(idx)
                            {
                                new_vertices -= 1;
                            }
                        }
                        score += (3 - new_vertices) as f32 * 20.0;
                        let mut t_center = glam::Vec3::ZERO;
                        let mut new_bounds: f32 = bounds;
                        for &idx in tri {
                            let pos = vertices[idx as usize].position.into_glam();
                            t_center += pos;
                            let to_center = (center - pos)
                                .length();
                            new_bounds = new_bounds.max(to_center);
                        }
                        score += bounds / new_bounds * 5.0;
                        t_center /= 3.0;
                        let diff = (s.origin() - t_center).length();
                        let norm = bounds / diff;
                        score += norm * 40.0;
                        score as u32
                    }).map(|(i, _)| i as u32)
                else {
                    break;
                };
                other.local_triangles.remove(take);
                let unmapped = other.triangles_umapped.remove(take);
                let local = unmapped.map(|global_idx| {
                    let (_, count) = other.hashed_vertices
                        .get_mut(&global_idx)
                        .unwrap();
                    *count -= 1;
                    if *count == 0 {
                        let (local, _) = other.hashed_vertices
                            .remove(&global_idx)
                            .unwrap();
                        for (l, _) in other.hashed_vertices.values_mut() {
                            if *l > local {
                                *l -= 1;
                            }
                        }
                        for triangle in &mut other.local_triangles {
                            *triangle = triangle.map(|idx|
                                if idx > local {
                                    idx - 1
                                } else {
                                    assert!(idx != local);
                                    idx
                                }
                            )
                        }
                    }
                    s.add_vertex(global_idx, vertices)
                });
                s.local_triangles.push(local);
                s.triangles_umapped.push(unmapped);
                s.recalculate_bounds(vertices);
            }
            for i in (0..other.local_vertices.len()).rev() {
                let v = other.local_vertices[i as usize];
                if !other.hashed_vertices.contains_key(&v)
                {
                    other.local_vertices.remove(i);
                }
            }
            (s, other)
        };
        if self.local_triangles.len() < other.local_triangles.len() {
            s(self, other)
        } else {
            s(other, self)
        }
    }

    #[inline]
    fn add_vertex(&mut self, global_idx: u32, vertices: &[Vertex]) -> u32
    {
        self.hashed_vertices
            .entry(global_idx)
            .and_modify(|(_, count)| {
                *count += 1
            }).or_insert_with(|| {
                let vertex = vertices[global_idx as usize];
                if self.local_vertices.len() < 3 {
                    self.origin_add += vertex.position.into_glam();
                }
                self.center_add += vertex.position.into_glam();
                self.center = self.center_add / self.local_vertices.len() as f32;
                let local = self.local_vertices.len();
                self.local_vertices.push(global_idx);
                (local, 1)
            }).0
    }

    fn recalculate_bounds(&mut self, vertices: &[Vertex]) {
        let center = self.center;
        let mut radius: f32 = 0.0;
        for &idx in &self.local_vertices {
            let to_center = (center - vertices[idx as usize].position.into_glam())
                .length();
            radius = radius.max(to_center);
        }
        self.bounding_sphere_radius = radius;
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub scale: glam::Vec3,
    pub translation: glam::Vec3,
    pub vertices: Vec<Vertex>,
    pub meshlets: Vec<Meshlet>,
}

impl Mesh {

    pub fn from_obj(
        name: &str,
        obj: &str,
        max_local_vertices: u32,
        max_local_primitives: u32,
    ) -> EventResult<Self> {
        type VertexIndices = (u32, Option<u32>, Option<u32>);
        let mut positions: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut textures: Vec<[f32; 2]> = vec![];
        let mut faces: Vec<[VertexIndices; 3]> = vec![];
        for line in obj.lines() {
            if line.starts_with("v ") {
                let mut pos = [0f32; 3];
                for (i, elem) in line.split(" ").skip(1).enumerate() {
                    pos[i] = str::parse(elem).context("parse error")?;
                }
                positions.push(pos);
            } else if line.starts_with("vn ") {
                let mut norm = [0f32; 3];
                for (i, elem) in line.split(" ").skip(1).enumerate() {
                    norm[i] = str::parse(elem).context("parse error")?;
                }
                normals.push(norm);
            } else if line.starts_with("vt ") {
                let mut tex = [0f32; 2];
                for (i, elem) in line.split(" ").skip(1).enumerate() {
                    tex[i] = str::parse(elem).context("parse error")?;
                }
                textures.push(tex);
            } else if line.starts_with("f ") {
                let mut face = [None; 4];
                for (i, elem) in line.split(" ").skip(1).enumerate() {
                    let mut iter = elem.split("/");
                    let pos: u32 = str::parse(iter.next().unwrap()).context("parse error")?;
                    let tex = iter.next().and_then(|str| str::parse::<u32>(str).ok()).map(|idx| idx -1);
                    let n = iter.next().and_then(|str| str::parse::<u32>(str).ok()).map(|idx| idx -1);
                    face[i] = Some((pos - 1, tex, n));
                }
                if face[3].is_some() {
                    let f1 = [face[0].unwrap(), face[1].unwrap(), face[2].unwrap()];
                    let f2 = [face[2].unwrap(), face[3].unwrap(), face[0].unwrap()];
                    faces.extend([f1, f2]);
                } else {
                    let f1 = [face[0].unwrap(), face[1].unwrap(), face[2].unwrap()];
                    faces.push(f1);
                }
            }
        }
        struct VertexTriangles {
            index: u32,
            touched_triangles: Vec<u32>,
        }
        struct Triangle {
            used: UnsafeCell<bool>,
            indices: [u32; 3],
            neighbors: IndexSet<u32>,
        }
        let mut vertices: Vec<Vertex> = vec![];
        let mut unique_vertices = IndexMap::new();
        let mut triangles: Vec<Triangle> = faces.into_iter()
            .enumerate()
            .map(|(tri_idx, verts)| {
                Triangle {
                    used: UnsafeCell::new(false),
                    indices: verts.map(|(pos, tex, n)| {
                        let new = Vertex {
                            position: positions[pos as usize].into(),
                            normal: n.map(|idx| normals[idx as usize].into()).unwrap_or_default(),
                            uv: tex.map(|idx| textures[idx as usize].into()).unwrap_or_default(),
                        };
                        unique_vertices.entry(new)
                            .and_modify(|VertexTriangles { touched_triangles, .. }| {
                                touched_triangles.push(tri_idx as u32);
                            }).or_insert_with(|| {
                                let idx = vertices.len();
                                vertices.push(new);
                                VertexTriangles {
                                    index: idx as u32,
                                    touched_triangles: vec![tri_idx as u32],
                                }
                            }).index
                    }),
                    neighbors: IndexSet::default(),
                }
            }).collect();
        for VertexTriangles { touched_triangles, .. } in unique_vertices.values() {
            for &index in touched_triangles {
                let neighbors = &mut triangles[index as usize].neighbors;
                neighbors.extend(touched_triangles);
            }
        }
        let mut meshlets = vec![];
        let mut current_meshlet = meshlets.push_mut(Meshlet::new(max_local_vertices, max_local_primitives));
        let mut triangle_maps = FixedBuffer::with_capacity(max_local_primitives);
        let mut queue = VecDeque::from([0u32]);
        loop {
            while let Some(idx) = queue.pop_front() {
                let triangle = &triangles[idx as usize];
                unsafe {
                    assert!(!*triangle.used.get());
                    *triangle.used.get() = true;
                }
                if current_meshlet.is_full() {
                    triangle_maps.clear();
                    current_meshlet = meshlets.push_mut(Meshlet::new(max_local_vertices, max_local_primitives));
                }
                let local_indices = triangle.indices.map(|global_idx| {
                    current_meshlet.add_vertex(global_idx, &vertices)
                });
                current_meshlet.local_triangles.push(local_indices);
                current_meshlet.triangles_umapped.push(triangle.indices);
                triangle_maps.push(idx);
                current_meshlet.recalculate_bounds(&vertices);
                let mut best_neighbor = None;
                let mut best_score = -1.0;
                for &idx in &triangle.neighbors {
                    let triangle = &triangles[idx as usize];
                    unsafe {
                        if *triangle.used.get() {
                            continue;
                        }
                    }
                    let mut score = 0f32;
                    let mut new_vertices = 3;
                    for idx in triangle.indices {
                        if current_meshlet.hashed_vertices
                            .contains_key(&idx)
                        {
                            new_vertices -= 1;
                        }
                    }
                    score += (3 - new_vertices) as f32 * 20.0;
                    let bounds = current_meshlet.bounding_sphere_radius();
                    let center = current_meshlet.center();
                    let mut new_bounds: f32 = bounds;
                    let mut t_center = glam::Vec3::ZERO;
                    for idx in triangle.indices {
                        let pos = vertices[idx as usize].position.into_glam();
                        t_center += pos;
                        let to_center = (center - pos)
                            .length();
                        new_bounds = new_bounds.max(to_center);
                    }
                    score += bounds / new_bounds * 5.0;
                    t_center /= 3.0;
                    let diff = (current_meshlet.origin() - t_center).length();
                    let norm = bounds / diff;
                    score += norm * 40.0;
                    if score > best_score {
                        best_neighbor = Some(idx);
                        best_score = score;
                    }
                }
                if let Some(idx) = best_neighbor {
                    queue.push_back(idx);
                }
            }
            let mut found_triangle = false;
            for &idx in &triangle_maps {
                let triangle = &triangles[idx as usize];
                let mut unused_neighbor = None;
                for &idx in &triangle.neighbors {
                    unsafe {
                        if !*triangles[idx as usize].used.get() {
                            unused_neighbor = Some(idx);
                        }
                    }
                }
                if let Some(idx) = unused_neighbor {
                    queue.push_back(idx);
                    found_triangle = true;
                    break;
                }
            }
            if found_triangle {
                continue
            }
            current_meshlet = meshlets.push_mut(Meshlet::new(max_local_vertices, max_local_primitives));
            triangle_maps.clear();
            let Some(idx) = triangles
                .iter_mut()
                .enumerate()
                .find_map(|(idx, tri)| {
                    (!*tri.used.get_mut()).then_some(idx as u32)
                }) else {
                break;
            };
            queue.push_back(idx);
        }
        let mut split_meshlets = AHashSet::default();
        let mut split_pairs = vec![];
        let mut iterations = 0;
        loop {
            for i in 0..meshlets.len().saturating_sub(1) {
                for j in i+1..meshlets.len() {
                    let a = &meshlets[i];
                    let b = &meshlets[j];
                    if a.should_split(b) {
                        if !split_meshlets.insert(i) {
                            continue
                        }
                        if !split_meshlets.insert(j) {
                            split_meshlets.remove(&i);
                            continue
                        }
                        split_pairs.push((i, j));
                    }
                }
            }
            if split_pairs.is_empty() {
                break;
            }
            for &(a, b) in split_pairs.iter().rev() {
                let b = meshlets.remove(b);
                let a = meshlets.remove(a);
                let (a, b) = a.split(b, &vertices);
                meshlets.push(a);
                meshlets.push(b);
            }
            iterations += 1;
            split_pairs.clear();
            //split_meshlets.clear();
        }
        log::info!("{name} meshlet count: {}", meshlets.len());
        log::info!("merge iterations: {iterations}");
        let average_triangle_count =
            meshlets
                .iter()
                .map(|meshlet| meshlet.local_triangles.len())
                .sum::<u32>() as f32 / meshlets.len() as f32;
        let average_vertex_count =
            meshlets
                .iter()
                .map(|meshlet| meshlet.local_vertices.len())
                .sum::<u32>() as f32 / meshlets.len() as f32;
        log::info!("{name} average primitive count: {average_triangle_count} / {max_local_primitives}");
        log::info!("{name} average vertex count: {average_vertex_count} / {max_local_vertices}");
        Ok(Self {
            scale: glam::vec3(1.0, 1.0, 1.0),
            translation: glam::vec3(0.0, 0.0, -3.0),
            vertices,
            meshlets,
        })
    }

    #[inline]
    pub fn with_scale(mut self, scale: glam::Vec3) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn with_translation(mut self, translation: glam::Vec3) -> Self {
        self.translation = translation;
        self
    }
}
