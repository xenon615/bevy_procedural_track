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

impl SplainParams {
    pub fn with_cyclic(mut self, cyclic: bool) -> Self {
        self.cyclic = cyclic;
        self
    }
}

impl Default for SplainParams {
    fn default() -> Self {
        Self {
            subdivision: 120,
            tension: 0.3,
            cyclic: false
        }
    }
}

// ---

pub fn track_mesh(points: &Vec<(Vec3, Vec3)>,  profile: impl  ElementProfile) -> Mesh {

    let mut verts:Vec<[f32; 3]> = vec![];
    let mut idxs: Vec<u32> = vec![];
    for i in 0 .. points.len() - 1 {
        let current = points[i];
        let next = points[ i + 1];
        profile.build(&current, &next, &mut verts, &mut idxs);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
        .with_inserted_attribute( Mesh::ATTRIBUTE_POSITION, verts)
        .with_inserted_indices(Indices::U32(idxs))
        .with_computed_normals()
}
