use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    mesh:: {Indices,  PrimitiveTopology},
};

pub mod profile;
pub use crate::profile::ElementProfile;

///Used for creation track mesh
// parameters:
///points: vector of tuples of position and binormal
///profile: profile of mesh cut (can be custom)
///start_normal and end_normal are normals of first cut and last cut respectively
pub fn track_mesh(points: &Vec<(Vec3, Vec3)>,  profile: impl  ElementProfile, start_normal: Option<Vec3>, end_normal: Option<Vec3>) -> Mesh {
    let mut verts:Vec<[f32; 3]> = vec![];
    let mut idxs: Vec<u32> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];
    let mut prev_cut = vec![];
    let mut idx = 0;
    let mut points_iter = points.iter().peekable();
    while let Some(current) = points_iter.next() {

        let normal = match points_iter.peek() {
            Some(_) if idx == 0  && start_normal.is_some() =>  {
                idx += 1;
                start_normal.unwrap()
            },
            Some(next) => next.0 - current.0,
            None => if let Some(normal) = end_normal  { normal} else {current.0 - points[points.len() -2].0}
        }.normalize();

        let cut = profile.cut(&current.0, &normal, &current.1);
        if !prev_cut.is_empty() {
            profile.build(&prev_cut, &cut, &mut verts, &mut idxs, &mut uvs);
        }
        prev_cut = cut;
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
