use leimu::{
    mem::vec::ArrayVec,
    error::Context,
    EventResult,
    EventError,
};

use super::shader_types::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

#[derive(Debug)]
pub struct Meshlet {
    pub local_vertices: ArrayVec<u32, 6>,
    pub local_triangles: ArrayVec<[u32; 3], 2>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub meshlets: Vec<Meshlet>,
}

impl Mesh {

    pub fn from_obj(
        obj: &str,
    ) -> EventResult<Self> {
        let mut positions: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut textures: Vec<[f32; 2]> = vec![];
        let mut faces: Vec<[(u32, u32, u32); 3]> = vec![];
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
                    let tex: u32 = str::parse(iter.next().unwrap()).context("parse error")?;
                    let n: u32 = str::parse(iter.next().unwrap()).context("parse error")?;
                    face[i] = Some((pos - 1, tex - 1, n -1));
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
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        for verts in faces {
            for (pos, tex, n) in verts {
                let new = Vertex {
                    position: positions[pos as usize].into(),
                    normal: normals[n as usize].into(),
                    uv: textures[tex as usize].into(),
                };
                let idx = 
                    if let Some(idx) = vertices
                        .iter()
                        .enumerate()
                        .find_map(|(i, &vert)| (vert == new).then_some(i as u32))
                    {
                        idx
                    } else {
                        let idx = vertices.len().try_into().unwrap();
                        vertices.push(new);
                        idx
                    };
                indices.push(idx);
            }
        }
        let mut i = 0;
        let mut meshlets = vec![];
        while i + 5 < indices.len() {
            let local_vertices: ArrayVec<_, _> = [
                indices[i], indices[i + 1], indices[i + 2],
                indices[i + 3], indices[i + 4], indices[i + 5],
            ].into();
            let mut unique_vertices = ArrayVec::<(u32, ArrayVec<_, _>), 6>::new();
            for (i, &vert) in local_vertices.iter().enumerate() {
                if let Some((_, local)) = unique_vertices
                    .iter_mut().find(|&&mut (idx, _)| idx == vert)
                {
                    local.push(i);
                } else {
                    let mut local = ArrayVec::<_, 6>::new();
                    local.push(i);
                    unique_vertices.push((vert, local));
                }
            }
            let local_vertices: ArrayVec<_, _> = unique_vertices
                .iter().map(|&(idx, _)| idx).collect();
            let mut local_triangles = ArrayVec::new();
            for prim in 0..2 {
                let idx_start = prim * 3;
                let mut local_indices = [0u32; 3];
                for (i, l) in local_indices.iter_mut().enumerate() {
                    let loc = idx_start + i;
                    let idx = unique_vertices
                        .iter()
                        .enumerate()
                        .find_map(|(idx, (_, local))| {
                            local.contains(&loc)
                            .then_some(idx as u32)
                        }).unwrap();
                    *l = idx;
                }
                local_triangles.push(local_indices);
            }
            meshlets.push(Meshlet {
                local_vertices,
                local_triangles,
            });
            i += 6;
        }
        if i < indices.len() {
            if indices.len() - i != 3 {
                return Err(EventError::just_context(format!(
                    "invalid remaining index count {}",
                    indices.len() - i
                )))
            }
            let local_vertices: ArrayVec<_, _> = [
                indices[i], indices[i + 1], indices[i + 3]
            ].into_iter().collect();
            let mut unique_vertices = ArrayVec::<(u32, ArrayVec<_, _>), 6>::new();
            for (i, &vert) in local_vertices.iter().enumerate() {
                if let Some((_, local)) = unique_vertices
                    .iter_mut().find(|&&mut (idx, _)| idx == vert)
                {
                    local.push(i);
                } else {
                    let mut local = ArrayVec::<_, 6>::new();
                    local.push(i);
                    unique_vertices.push((vert, local));
                }
            }
            let mut local_indices = [0u32; 3];
            for (i, l) in local_indices.iter_mut().enumerate() {
                let loc = i;
                let idx = unique_vertices
                    .iter()
                    .enumerate()
                    .find_map(|(idx, (_, local))| {
                        local.contains(&loc)
                        .then_some(idx as u32)
                    }).unwrap();
                *l = idx;
            }
            let mut local_triangles = ArrayVec::new();
            local_triangles.push(local_indices);
            meshlets.push(Meshlet {
                local_vertices,
                local_triangles,
            });
        }
        Ok(Self {
            vertices,
            meshlets,
        })
    }
}
