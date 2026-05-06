use std::{
    collections::{VecDeque},
};

use core::{
    slice,
    hash::Hash,
    cell::UnsafeCell,
};

use leimu::{
    core::AsBytes,
    default,
    mem::{
        reserve::{ReserveError, ReservePolicy},
        vec::{ArrayVec, StdVecBase},
    },
};

use ahash::{AHashMap, AHashSet};
use smallvec::SmallVec;

type IndexMap<K, V> = indexmap::IndexMap<K, V, ahash::RandomState>;
type IndexSet<T> = indexmap::IndexSet<T, ahash::RandomState>;

pub struct FixedPolicy;

unsafe impl ReservePolicy<u32> for FixedPolicy {

    #[inline]
    fn can_grow() -> bool {
        false
    }

    #[inline]
    fn grow(current: u32, required: usize) -> Result<u32, ReserveError<()>> {
        if current == 0 {
            return Ok(required as u32)
        }
        if required > current as usize {
            Err(ReserveError::max_capacity_exceeded(current, required, ()))
        } else {
            Ok(current)
        }
    }

    #[inline]
    fn grow_infallible(current: u32, required: usize) -> u32 {
        if current == 0 {
            return required as u32
        }
        if required > current as usize {
            panic!("maximum capacity of {current} exceeded")
        }
        current
    }
}

pub type FixedBuffer<T> = StdVecBase<T, u32, FixedPolicy>;

pub trait Vertex: Clone + Copy + PartialEq + Eq + Hash {

    type Vec3: From<[f32; 3]> + Into<[f32; 3]>;

    fn get_position(&self) -> Self::Vec3;
    fn set_position(&mut self, pos: Self::Vec3);
}

#[derive(Default, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl Vec3 {

    const ZERO: Self = vec3(0.0, 0.0, 0.0);

    #[inline]
    fn length(&self) -> f32 {
        (
            self.x * self.x +
            self.y * self.y +
            self.z * self.z
        ).sqrt()
    }
}

impl ::core::ops::Add for Vec3 {

    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ::core::ops::AddAssign for Vec3 {

    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ::core::ops::Sub for Vec3 {

    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ::core::ops::Div<f32> for Vec3 {

    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self  {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ::core::ops::DivAssign<f32> for Vec3 {

    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl From<[f32; 3]> for Vec3
{
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<Vec3> for [f32; 3]
{
    #[inline]
    fn from(value: Vec3) -> Self {
        [value.x, value.y, value.z]
    }
}

#[derive(Clone, Copy)]
struct MeshletTriangle {
    local: [u32; 3],
    global: [u32; 3],
}

/// (count, touching triangles)
type HashedTriangle = (u32, SmallVec<[[u32; 3]; 4]>);

#[derive(Clone)]
pub struct MeshletBuilder {
    local_vertices: FixedBuffer<u32>,
    hashed_vertices: AHashMap<u32, (u32, u32)>,
    edges: IndexMap<[u32; 2], ArrayVec<[u32; 3], 2>>,
    triangles: FixedBuffer<MeshletTriangle>,
    origin_add: Vec3,
    center_add: Vec3,
    center: Vec3,
    bounding_sphere_radius: f32,
}

impl MeshletBuilder {

    fn new(
        max_local_vertices: u32,
        max_local_primitives: u32,
    ) -> Self {
        Self {
            local_vertices: FixedBuffer::with_capacity(max_local_vertices),
            hashed_vertices: AHashMap::new(),
            edges: IndexMap::default(),
            triangles: FixedBuffer::with_capacity(max_local_primitives),
            origin_add: default(),
            center_add: default(),
            center: default(),
            bounding_sphere_radius: 0.0,
        }
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.local_vertices.len() + 3 > self.local_vertices.capacity() ||
        self.triangles.len() == self.triangles.capacity()
    }

    #[inline]
    fn center(&self) -> Vec3 {
        self.center
    }

    #[inline]
    fn origin(&self) -> Vec3 {
        self.origin_add / 3.0
    }

    #[inline]
    fn bounding_sphere_radius(&self) -> f32 {
        self.bounding_sphere_radius
    }

    #[inline]
    fn add_vertex(&mut self, global_idx: u32, vertices: &[impl Vertex]) -> u32
    {
        self.hashed_vertices
            .entry(global_idx)
            .or_insert_with(|| {
                let vertex = vertices[global_idx as usize];
                let pos: Vec3 = vertex.get_position().into().into();
                if self.local_vertices.len() < 3 {
                    self.origin_add += pos;
                }
                self.center_add += pos;
                self.center = self.center_add / self.local_vertices.len() as f32;
                let local = self.local_vertices.len();
                self.local_vertices.push(global_idx);
                (local, default())
            }).0
    }

    #[inline]
    fn add_triangle(&mut self, global: [u32; 3], local: [u32; 3]) {
        for idx in global {
            let (_, tris) = self.hashed_vertices.get_mut(&idx).unwrap();
            *tris += 1;
        }
        for mut edge in [
            [global[0], global[1]],
            [global[1], global[2]],
            [global[2], global[0]],
        ] {
            if edge[0] > edge[1] {
                edge.swap(0, 1);
            }
            self.edges
                .entry(edge)
                .and_modify(|tri| {
                    tri.push(global);
                }).or_insert_with(|| {
                    (0..1).map(|_| global).collect()
                });
        }
        self.triangles.push(MeshletTriangle { local, global });
    }

    #[inline]
    fn should_split(&self, other: &Self, largest_diff: u32) -> Option<u32> {
        let n_a = self.triangles.len();
        let n_b = other.triangles.len();
        let diff = n_a.saturating_sub(n_b).max(n_b.saturating_sub(n_a));
        if diff <= largest_diff {
            return None
        }
        if self.triangles.len() < other.triangles.len() {
            for idx in &self.local_vertices {
                if other.hashed_vertices.contains_key(idx) {
                    return Some(diff)
                }
            }
        } else {
            for idx in &other.local_vertices {
                if self.hashed_vertices.contains_key(idx) {
                    return Some(diff)
                }
            }
        }
        None
    }

    #[inline]
    fn split(self, other: Self, vertices: &[impl Vertex]) -> (u32, Self, Self)
    {
        let threshold = self.triangles.capacity() / 4;
        let s = |mut s: Self, mut other: Self| -> (u32, Self, Self) {
            let mut outline_hash1: IndexSet<[u32; 2]> = default();
            let mut outline_hash2: IndexSet<u32> = default();
            let mut num_iterations = 0;
            while s.triangles.len() <= threshold &&
                s.local_vertices.len() + 2 < s.local_vertices.capacity()
            {
                let Some(take) = other.triangles
                    .iter()
                    .enumerate()
                    .filter(|(_, tri)| {
                        let global = tri.global;
                        if global
                            .iter()
                            .filter(|g| s.hashed_vertices.contains_key(g))
                            .count() == 0
                        {
                            return false
                        }
                        let neighbors: ArrayVec<_, 3> = [
                            [global[0], global[1]],
                            [global[1], global[2]],
                            [global[2], global[0]],
                        ].into_iter().filter_map(|mut edge| {
                            if edge[0] > edge[1] {
                                edge.swap(0, 1);
                            }
                            let triangles = other.edges.get(&edge).unwrap();
                            triangles.iter().find(|&&tri| tri != global)
                                .copied()
                        }).collect();
                        let skip_tri = global;
                        let mut tris: ArrayVec<_, 12> = neighbors
                            .map(|&global| {
                                let tris: ArrayVec<_, 3> = [
                                    [global[0], global[1]],
                                    [global[1], global[2]],
                                    [global[2], global[0]],
                                ].into_iter().filter_map(|mut edge| {
                                    if edge[0] > edge[1] {
                                        edge.swap(0, 1);
                                    }
                                    let triangles = other.edges.get(&edge).unwrap();
                                    triangles.iter().find(|&&tri| tri != global)
                                        .copied().and_then(|tri| (tri != skip_tri).then_some(tri))
                                }).collect();
                                tris
                            }).into_iter().flatten().collect();
                        tris.extend(neighbors);
                        if tris.is_empty() {
                            return false
                        }
                        outline_hash1.clear();
                        const E0: u8 = 0x1;
                        const E1: u8 = 0x2;
                        const E2: u8 = 0x4;
                        for i in 0..tris.len() {
                            let mut found_edges = 0;
                            let a = tris[i];
                            let v0 = a[0];
                            let v1 = a[1];
                            let v2 = a[2];
                            let mut e0 = [v0, v1];
                            let mut e1 = [v1, v2];
                            let mut e2 = [v2, v0];
                            for j in 0..tris.len() {
                                let b = tris[j];
                                if a != b {
                                    let x = b.contains(&v0);
                                    let y = b.contains(&v1);
                                    let z = b.contains(&v2);
                                    if x && y {
                                        found_edges |= E0;
                                    }
                                    if y && z {
                                        found_edges |= E1;
                                    }
                                    if z && x {
                                        found_edges |= E2;
                                    }
                                }
                            }
                            if found_edges.count_ones() != 3 {
                                let flags = (found_edges ^ !0) & (E0 | E1 | E2);
                                if flags & E0 == E0 {
                                    if e0[0] > e0[1] {
                                        e0.swap(0, 1);
                                    }
                                    outline_hash1.insert(e0);
                                }
                                if flags & E1 == E1 {
                                    if e1[0] > e1[1] {
                                        e1.swap(0, 1);
                                    }
                                    outline_hash1.insert(e1);
                                }
                                if flags & E2 == E2 {
                                    if e2[0] > e2[1] {
                                        e2.swap(0, 1);
                                    }
                                    outline_hash1.insert(e2);
                                }
                            }
                        }
                        let n = outline_hash1.len();
                        outline_hash2.clear();
                        let mut idx = Some(0);
                        while let Some(next) = idx.take() &&
                            let Some(next) = outline_hash1.swap_remove_index(next)
                        {
                            let this_a = next[0];
                            let this_b = next[1];
                            for (i, &[a, b]) in outline_hash1.iter().enumerate() {
                                if this_a == a || this_a == b
                                {
                                    if !outline_hash2.insert(this_b) {
                                        return false
                                    }
                                    idx = Some(i);
                                    break;
                                }
                                if this_b == a || this_b == b
                                {
                                    if !outline_hash2.insert(this_a) {
                                        return false
                                    }
                                    idx = Some(i);
                                    break;
                                }
                            }
                        }
                        if outline_hash1.is_empty() {
                            println!("hash1: {n}, tris: {}", tris.len());
                        } else {
                            println!("hash1 not empty: {}, hash1: {n} tris: {}", outline_hash1.len(), tris.len());
                        }
                        outline_hash1.is_empty()
                    }).max_by_key(|&(_, tri)| {
                        let bounds = s.bounding_sphere_radius();
                        let center = s.center();
                        let mut score = 0.0;
                        let mut new_vertices = 3;
                        for idx in tri.global {
                            if s.hashed_vertices
                                .contains_key(&idx)
                            {
                                new_vertices -= 1;
                            }
                        }
                        score += (3 - new_vertices) as f32 * 20.0;
                        let mut t_center = Vec3::ZERO;
                        let mut new_bounds: f32 = bounds;
                        for idx in tri.global {
                            let pos: Vec3 = vertices[idx as usize]
                                .get_position()
                                .into()
                                .into();
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
                let tri = other.triangles.remove(take);
                for mut edge in [
                    [tri.global[0], tri.global[1]],
                    [tri.global[1], tri.global[2]],
                    [tri.global[2], tri.global[0]],
                ] {
                    if edge[0] > edge[1] {
                        edge.swap(0, 1);
                    }
                    let edges = other.edges
                        .get_mut(&edge)
                        .unwrap();
                    edges.retain(|&global| tri.global == global);
                }
                let local = tri.global.map(|global_idx| {
                    let (_, tris) = other.hashed_vertices
                        .get_mut(&global_idx)
                        .unwrap();
                    *tris -= 1;
                    if *tris == 0 {
                        let (local, _) = other.hashed_vertices
                            .remove(&global_idx)
                            .unwrap();
                        for (l, _) in other.hashed_vertices.values_mut() {
                            if *l > local {
                                *l -= 1;
                            }
                        }
                        for triangle in &mut other.triangles {
                            triangle.local = triangle.local.map(|idx|
                                if idx > local {
                                    idx - 1
                                } else {
                                    assert!(idx != local);
                                    idx
                                }
                            )
                        }
                        other.local_vertices.remove(local);
                    }
                    s.add_vertex(global_idx, vertices)
                });
                s.add_triangle(tri.global, local);
                s.recalculate_bounds(vertices);
                num_iterations += 1;
            }
            other.recalculate_bounds(vertices);
            (num_iterations, s, other)
        };
        if self.triangles.len() < other.triangles.len() {
            s(self, other)
        } else {
            s(other, self)
        }
    }

    fn recalculate_bounds(&mut self, vertices: &[impl Vertex]) {
        let center = self.center;
        let mut radius: f32 = 0.0;
        for &idx in &self.local_vertices {
            let to_center = (center - vertices[idx as usize].get_position().into().into())
                .length();
            radius = radius.max(to_center);
        }
        self.bounding_sphere_radius = radius;
    }

    fn finalize(self) -> Meshlet {
        Meshlet {
            local_vertices: self.local_vertices,
            local_triangles: self.triangles
                .into_iter()
                .map(|tri| tri.local)
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Meshlet {
    local_vertices: FixedBuffer<u32>,
    local_triangles: FixedBuffer<[u32; 3]>,
}

impl Meshlet {

    #[inline]
    pub fn local_vertices(&self) -> &[u32] {
        &self.local_vertices
    }


    #[inline]
    pub fn local_vertices_full_as_bytes(&self) -> &[u8] {
        (unsafe {
            slice::from_raw_parts(
                self.local_vertices.as_ptr(),
                self.local_vertices.capacity() as usize,
            )
        }).as_bytes()
    }

    #[inline]
    pub fn local_triangles(&self) -> &[[u32; 3]] {
        &self.local_triangles
    }

    #[inline]
    pub fn local_triangles_full_as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.local_triangles.as_ptr(),
                self.local_triangles.capacity() as usize,
           )
        }.as_bytes()
    }
}

#[derive(Clone)]
pub struct Mesh<V: Vertex> {
    vertices: Vec<V>,
    meshlets: Box<[Meshlet]>,
}

impl<V: Vertex> Mesh<V> {

    pub fn new<F>(
        name: &str,
        faces: F,
        max_local_vertices: u32,
        max_local_primitives: u32,
    ) -> Self
        where F: IntoIterator<Item = [V; 3]>
    {
        struct VertexTriangles {
            index: u32,
            touched_triangles: Vec<u32>,
        }
        struct Triangle {
            used: UnsafeCell<bool>,
            indices: [u32; 3],
            neighbors: IndexSet<u32>,
        }
        let mut vertices: Vec<V> = vec![];
        let mut unique_vertices = IndexMap::default();
        let mut triangles: Vec<Triangle> = faces.into_iter()
            .enumerate()
            .map(|(tri_idx, verts)| {
                Triangle {
                    used: UnsafeCell::new(false),
                    indices: verts.map(|new| {
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
        let mut current_meshlet = meshlets.push_mut(MeshletBuilder::new(
            max_local_vertices, max_local_primitives
        ));
        let mut triangle_maps = FixedBuffer::with_capacity(max_local_primitives);
        let mut queue = VecDeque::from([0u32]);
        loop {
            while let Some(idx) = queue.pop_front() {
                let triangle = &triangles[idx as usize];
                unsafe {
                    *triangle.used.get() = true;
                }
                if current_meshlet.is_full() {
                    current_meshlet = meshlets.push_mut(MeshletBuilder::new(
                        max_local_vertices, max_local_primitives
                    ));
                    triangle_maps.clear();
                }
                let local_indices = triangle.indices.map(|global_idx| {
                    current_meshlet.add_vertex(global_idx, &vertices)
                });
                current_meshlet.add_triangle(triangle.indices, local_indices);
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
                    let mut t_center = Vec3::ZERO;
                    for idx in triangle.indices {
                        let pos = vertices[idx as usize]
                            .get_position()
                            .into()
                            .into();
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
            current_meshlet = meshlets.push_mut(MeshletBuilder::new(
                max_local_vertices, max_local_primitives
            ));
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
        let mut iterations = 0;
        let mut nul_in_row = 0;
        let mut split_meshlets = AHashSet::default();
        let mut split_pairs = vec![];
        loop {
            for i in 0..meshlets.len() {
                if split_meshlets.contains(&i) {
                    continue
                }
                let mut best_j: Option<usize> = None;
                let mut largest_primitive_diff = 0;
                for j in 0..meshlets.len() {
                    if i == j { continue }
                    let a = &meshlets[i];
                    let b = &meshlets[j];
                    if !split_meshlets.contains(&j) &&
                        let Some(diff) = a.should_split(b, largest_primitive_diff)
                    {
                        largest_primitive_diff = diff;
                        best_j = Some(j);
                    }
                }
                if let Some(j) = best_j {
                    split_meshlets.insert(i);
                    split_meshlets.insert(j);
                    split_pairs.push((i, j));
                }
            }
            if split_pairs.is_empty() {
                break;
            }
            let mut total_iterations = 0;
            for &(a, b) in split_pairs.iter().rev() {
                let b = meshlets.remove(b);
                let a = meshlets.remove(a);
                let (i, a, b) = a.split(b, &vertices);
                total_iterations += i;
                meshlets.push(a);
                meshlets.push(b);
            }
            iterations += 1;
            if total_iterations == 0 {
                nul_in_row += 1;
            } else {
                nul_in_row = 0;
            }
            if nul_in_row > 20 {
                break;
            }
            split_pairs.clear();
            split_meshlets.clear();
        }
        log::info!("{name} meshlet count: {}", meshlets.len());
        log::info!("merge iterations: {iterations}");
        let average_triangle_count =
            meshlets
                .iter()
                .map(|meshlet| meshlet.triangles.len())
                .sum::<u32>() as f32 / meshlets.len() as f32;
        let average_vertex_count =
            meshlets
                .iter()
                .map(|meshlet| meshlet.local_vertices.len())
                .sum::<u32>() as f32 / meshlets.len() as f32;
        log::info!("{name} average primitive count: {average_triangle_count} / {max_local_primitives}");
        log::info!("{name} average vertex count: {average_vertex_count} / {max_local_vertices}");
        Self {
            vertices,
            meshlets: meshlets.into_iter()
                .map(|m| m.finalize())
                .collect(),
        }
    }

    #[inline]
    pub fn vertices(&self) -> &[V] {
        &self.vertices
    }

    #[inline]
    pub fn meshlets(&self) -> &[Meshlet] {
        &self.meshlets
    }

    pub fn recenter(&mut self) {
        let mut avg = Vec3::ZERO;
        for vertex in &self.vertices {
            avg += vertex.get_position().into().into();
        }
        avg /= self.vertices.len() as f32;
        for vertex in &mut self.vertices {
            let pos: Vec3 = vertex.get_position().into().into();
            vertex.set_position(Into::<[f32; 3]>::into(pos - avg).into());
        }
    }
}
