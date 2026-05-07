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
struct MeshletPrimitive {
    local: [u32; 3],
    global: [u32; 3],
}

#[derive(Clone)]
pub struct MeshletBuilder {
    local_vertices: FixedBuffer<u32>,
    hashed_vertices: AHashMap<u32, (u32, u32)>,
    edges: IndexMap<[u32; 2], ArrayVec<[u32; 3], 2>>,
    primitives: FixedBuffer<MeshletPrimitive>,
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
            primitives: FixedBuffer::with_capacity(max_local_primitives),
            origin_add: default(),
            center_add: default(),
            center: default(),
            bounding_sphere_radius: 0.0,
        }
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.local_vertices.len() + 3 > self.local_vertices.capacity() ||
        self.primitives.len() == self.primitives.capacity()
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
    fn add_primitive(&mut self, global: [u32; 3], local: [u32; 3]) {
        for idx in global {
            let (_, prims) = self.hashed_vertices.get_mut(&idx).unwrap();
            *prims += 1;
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
                .and_modify(|prims| {
                    prims.push(global);
                }).or_insert_with(|| {
                    (0..1).map(|_| global).collect()
                });
        }
        self.primitives.push(MeshletPrimitive { local, global });
    }

    #[inline]
    fn should_balance(&self, other: &Self, largest_diff: u32) -> Option<u32> {
        let n_a = self.primitives.len();
        let n_b = other.primitives.len();
        let diff = n_a.saturating_sub(n_b).max(n_b.saturating_sub(n_a));
        if diff <= largest_diff {
            return None
        }
        if self.primitives.len() < other.primitives.len() {
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
    fn balance(self, other: Self, vertices: &[impl Vertex]) -> (u32, Self, Self)
    {
        let threshold = self.primitives.capacity() / 4;
        let s = |mut s: Self, mut other: Self| -> (u32, Self, Self) {
            let mut num_iterations = 0;
            while s.primitives.len() <= threshold &&
                s.local_vertices.len() + 2 < s.local_vertices.capacity()
            {
                let Some(take) = other.primitives
                    .iter()
                    .enumerate()
                    .filter(|(_, prim)| {
                        prim.global
                            .iter()
                            .filter(|id| {
                                s.hashed_vertices.contains_key(id)
                            }).count() >= 2
                    }).max_by_key(|&(_, prim)| {
                        let bounds = s.bounding_sphere_radius();
                        let center = s.center();
                        let mut score = 0.0;
                        let mut new_vertices = 3;
                        for idx in prim.global {
                            if s.hashed_vertices
                                .contains_key(&idx)
                            {
                                new_vertices -= 1;
                            }
                        }
                        score += (3 - new_vertices) as f32 * 20.0;
                        let mut t_center = Vec3::ZERO;
                        let mut new_bounds: f32 = bounds;
                        for idx in prim.global {
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
                let prim = other.primitives.remove(take);
                for mut edge in [
                    [prim.global[0], prim.global[1]],
                    [prim.global[1], prim.global[2]],
                    [prim.global[2], prim.global[0]],
                ] {
                    if edge[0] > edge[1] {
                        edge.swap(0, 1);
                    }
                    let edges = other.edges
                        .get_mut(&edge)
                        .unwrap();
                    edges.retain(|&global| prim.global == global);
                }
                let local = prim.global.map(|global_idx| {
                    let (_, prims) = other.hashed_vertices
                        .get_mut(&global_idx)
                        .unwrap();
                    *prims -= 1;
                    if *prims == 0 {
                        let (local, _) = other.hashed_vertices
                            .remove(&global_idx)
                            .unwrap();
                        for (l, _) in other.hashed_vertices.values_mut() {
                            if *l > local {
                                *l -= 1;
                            }
                        }
                        for primitive in &mut other.primitives {
                            primitive.local = primitive.local.map(|idx|
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
                s.add_primitive(prim.global, local);
                s.recalculate_bounds(vertices);
                num_iterations += 1;
            }
            other.recalculate_bounds(vertices);
            (num_iterations, s, other)
        };
        if self.primitives.len() < other.primitives.len() {
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
            local_primitives: self.primitives
                .into_iter()
                .map(|prim| prim.local)
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Meshlet {
    local_vertices: FixedBuffer<u32>,
    local_primitives: FixedBuffer<[u32; 3]>,
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
    pub fn local_primitives(&self) -> &[[u32; 3]] {
        &self.local_primitives
    }

    #[inline]
    pub fn local_primitives_full_as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.local_primitives.as_ptr(),
                self.local_primitives.capacity() as usize,
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
        struct VertexPrimitives {
            index: u32,
            touched_primitives: Vec<u32>,
        }
        struct Triangle {
            used: UnsafeCell<bool>,
            indices: [u32; 3],
            neighbors: IndexSet<u32>,
        }
        let mut vertices: Vec<V> = vec![];
        let mut unique_vertices = IndexMap::default();
        let mut primitives: Vec<Triangle> = faces.into_iter()
            .enumerate()
            .map(|(prim_idx, verts)| {
                Triangle {
                    used: UnsafeCell::new(false),
                    indices: verts.map(|new| {
                        unique_vertices.entry(new)
                            .and_modify(|VertexPrimitives { touched_primitives, .. }| {
                                touched_primitives.push(prim_idx as u32);
                            }).or_insert_with(|| {
                                let idx = vertices.len();
                                vertices.push(new);
                                VertexPrimitives {
                                    index: idx as u32,
                                    touched_primitives: vec![prim_idx as u32],
                                }
                            }).index
                    }),
                    neighbors: IndexSet::default(),
                }
            }).collect();
        for VertexPrimitives { touched_primitives, .. } in unique_vertices.values() {
            for &index in touched_primitives {
                let neighbors = &mut primitives[index as usize].neighbors;
                neighbors.extend(touched_primitives);
            }
        }
        let mut meshlets = vec![];
        let mut current_meshlet = meshlets.push_mut(MeshletBuilder::new(
            max_local_vertices, max_local_primitives
        ));
        let mut primitive_maps = FixedBuffer::with_capacity(max_local_primitives);
        let mut queue = VecDeque::from([0u32]);
        loop {
            while let Some(idx) = queue.pop_front() {
                let primitive = &primitives[idx as usize];
                unsafe {
                    *primitive.used.get() = true;
                }
                if current_meshlet.is_full() {
                    current_meshlet = meshlets.push_mut(MeshletBuilder::new(
                        max_local_vertices, max_local_primitives
                    ));
                    primitive_maps.clear();
                }
                let local_indices = primitive.indices.map(|global_idx| {
                    current_meshlet.add_vertex(global_idx, &vertices)
                });
                current_meshlet.add_primitive(primitive.indices, local_indices);
                primitive_maps.push(idx);
                current_meshlet.recalculate_bounds(&vertices);
                let mut best_neighbor = None;
                let mut best_score = -1.0;
                for &idx in &primitive.neighbors {
                    let primitive = &primitives[idx as usize];
                    unsafe {
                        if *primitive.used.get() {
                            continue;
                        }
                    }
                    let mut score = 0f32;
                    let mut new_vertices = 3;
                    for idx in primitive.indices {
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
                    for idx in primitive.indices {
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
            let mut found_primitive = false;
            for &idx in &primitive_maps {
                let primitive = &primitives[idx as usize];
                let mut unused_neighbor = None;
                for &idx in &primitive.neighbors {
                    unsafe {
                        if !*primitives[idx as usize].used.get() {
                            unused_neighbor = Some(idx);
                        }
                    }
                }
                if let Some(idx) = unused_neighbor {
                    queue.push_back(idx);
                    found_primitive = true;
                    break;
                }
            }
            if found_primitive {
                continue
            }
            current_meshlet = meshlets.push_mut(MeshletBuilder::new(
                max_local_vertices, max_local_primitives
            ));
            primitive_maps.clear();
            let Some(idx) = primitives 
                .iter_mut()
                .enumerate()
                .find_map(|(idx, prim)| {
                    (!*prim.used.get_mut()).then_some(idx as u32)
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
                        let Some(diff) = a.should_balance(b, largest_primitive_diff)
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
                let (i, a, b) = a.balance(b, &vertices);
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
        log::info!("balance iterations: {iterations}");
        let average_primitive_count =
            meshlets
                .iter()
                .map(|meshlet| meshlet.primitives.len())
                .sum::<u32>() as f32 / meshlets.len() as f32;
        let average_vertex_count =
            meshlets
                .iter()
                .map(|meshlet| meshlet.local_vertices.len())
                .sum::<u32>() as f32 / meshlets.len() as f32;
        log::info!("{name} average primitive count: {average_primitive_count} / {max_local_primitives}");
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
