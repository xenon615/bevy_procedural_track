use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    mesh:: {Indices,  PrimitiveTopology},
};

pub mod profile;
use crate::profile::ElementProfile;
pub struct SplainParams {
    pub subdivision: usize,
    pub tension: f32,
    pub cyclic: bool
}

// ---

pub fn track_mesh(points: &Vec<(Vec3, Vec3)>,  profile: impl  ElementProfile, cyclic: bool) -> Mesh {

    let mut verts:Vec<[f32; 3]> = vec![];
    let mut idxs: Vec<u32> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];
    let mut prev_cut = vec![];

    for i  in 0 .. points.len() - 1   {
        let current = points[i];
        let next = points[i + 1];

        let cut = profile.cut(&current.0, &(next.0 - current.0).normalize(), &current.1);
        if prev_cut.is_empty() {
          prev_cut = profile.cut(&current.0, &(next.0 - current.0).normalize(), &current.1);
        }
        profile.build(&prev_cut, &cut, &mut verts, &mut idxs, &mut uvs);
        prev_cut = cut;
    }

    if cyclic {
        let current = points[0];
        let next = points[1];
        let cut = profile.cut(&current.0, &(next.0 - current.0).normalize(), &current.1);
        profile.build(&prev_cut, &cut, &mut verts, &mut idxs, &mut uvs);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
    .with_inserted_attribute( Mesh::ATTRIBUTE_POSITION, verts)
    .with_inserted_attribute( Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(idxs))
    .with_computed_normals()
}
