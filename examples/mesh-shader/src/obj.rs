use leimu::{EventResult, error::Context};

use core::hash::{self, Hash};

use crate::shader_types::*;

use leimu::core::AsBytes;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    _pad: [u8; 8],
}

impl Hash for Vertex {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes()[0..40].hash(state);
    }
}

impl PartialEq for Vertex {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes()[0..40] ==
        other.as_bytes()[0..40]
    }
}

impl Eq for Vertex {}

impl crate::meshlet::Vertex for Vertex {

    type Vec3 = Vec3;

    #[inline]
    fn get_position(&self) -> Self::Vec3 {
        self.position
    }

    #[inline]
    fn set_position(&mut self, pos: Self::Vec3) {
        self.position = pos;
    }
}

pub fn parse(
    obj: &str,
) -> EventResult<Vec<[Vertex; 3]>> {
    type VertexIndices = (u32, Option<u32>, Option<u32>); 
    let mut positions: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut textures: Vec<[f32; 2]> = vec![];
    let mut faces: Vec<[Vertex; 3]> = vec![];
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
            let make_vertex = |p: u32, t: Option<u32>, n: Option<u32>| -> Vertex {
                Vertex {
                    position: positions[p as usize].into(),
                    uv: t.map(|idx| textures[idx as usize]).unwrap_or_default().into(),
                    normal: n.map(|idx| normals[idx as usize]).unwrap_or_default().into(),
                    _pad: [0; 8],
                }
            };
            let make_face = |f: [VertexIndices; 3]| -> [Vertex; 3] {
                f.map(|idx| make_vertex(idx.0, idx.1, idx.2))
            };
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
                faces.extend([make_face(f1), make_face(f2)]);
            } else {
                let f1 = [face[0].unwrap(), face[1].unwrap(), face[2].unwrap()];
                faces.push(make_face(f1));
            }
        }
    }
    Ok(faces)
}
