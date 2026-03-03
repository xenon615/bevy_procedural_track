use bevy:: {
    asset::RenderAssetUsages,
    color::palettes::css,
    mesh:: {Indices,  PrimitiveTopology},
    pbr::wireframe::Wireframe,
    prelude::*
};

use crate::functions:: {convex_hull, modify_convex_hull};
use crate::profile::{ ElementProfile, EpBox, EpFlat, EpSquareChannel };

pub struct TrackPlugin;
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
        ;
    }
}

// ---

fn startup (
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let blue_mat = materials.add(Color::from(css::ROYAL_BLUE));
    let red_mat = materials.add(Color::from(css::CORAL));
    let green_mat = materials.add(Color::from(css::LEMON_CHIFFON));

    let subdivisions = 120;
    let path = convex_hull(10, vec3(10., 0., 10.));
    let path_m = modify_convex_hull(&path);

    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(path.clone()))),
        MeshMaterial3d(red_mat.clone())
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(path_m.clone()))),
        MeshMaterial3d(blue_mat.clone())
    ));

    let cr = CubicCardinalSpline::new(0.3, path_m).to_curve().unwrap();

    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(cr.iter_positions(subdivisions)))),
        MeshMaterial3d(green_mat.clone())
    ));

    let mut verts:Vec<[f32; 3]> = vec![];
    let mut idxs: Vec<u32> = vec![];

    let itr = cr.iter_positions(subdivisions)
        .zip(cr.iter_velocities(subdivisions))
        .map(| ( p, v ) | ( p, -v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    // let profile = EpFlat{half_width: 1.};
    // let profile = EpBox{half_width: 1., half_height: 0.5};
    let profile = EpSquareChannel{half_width: 1., height: 1., depth: 0.8, border_width: 0.1};

    for i in 0 .. itr.len() - 1 {
        let current = itr[i];
        let next = if i < itr.len() - 2  {itr[ i + 1]} else {itr[0]};
        profile.build(&current, &next, &mut verts, &mut idxs);
    }

    let road_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
        .with_inserted_attribute( Mesh::ATTRIBUTE_POSITION, verts)
        .with_inserted_indices(Indices::U32(idxs))
        .with_computed_normals()
    ;


    let mesh = meshes.add(road_mesh);

    cmd.spawn((
        Transform::from_xyz(0., 0., 0.)
        ,
        Mesh3d(mesh.clone()),
        MeshMaterial3d(blue_mat.clone()),
        Wireframe,
        // ColliderConstructor::TrimeshFromMesh,
        // RigidBody::Static
    ));

}
