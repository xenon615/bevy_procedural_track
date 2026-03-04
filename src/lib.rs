use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    mesh:: {Indices,  PrimitiveTopology},
};

mod profile;
use crate::profile::ElementProfile;
// pub struct ProcedurallTrackPlugin;
// impl Plugin for ProcedurallTrackPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_observer(add_track)
//         ;
//     }
// }

// #[derive(Component)]
// pub struct ProceduralTrack;


// fn add_track(
//     _tr: On<Add, ProceduralTrack >
// ) {
//     println!("added");

// }

pub struct SplainParams {
    subdivision: usize,
    tension: f32
}
impl Default for SplainParams {
    fn default() -> Self {
        Self {
            subdivision: 120,
            tension: 0.3
        }
    }
}

pub fn get_mesh(points: Vec<Vec3>, profile: impl  ElementProfile, spline_params: SplainParams) -> Mesh {

    let mut verts:Vec<[f32; 3]> = vec![];
    let mut idxs: Vec<u32> = vec![];

    let cr = CubicCardinalSpline::new(spline_params.tension, points).to_curve().unwrap();

    let itr = cr.iter_positions(spline_params.subdivision)
        .zip(cr.iter_velocities(spline_params.subdivision))
        .map(| ( p, v ) | ( p, -v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    for i in 0 .. itr.len() - 1 {
        let current = itr[i];
        let next = if i < itr.len() - 2  {itr[ i + 1]} else {itr[0]};
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
