// An example demonstrating the connection of a track and a cuboid

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    pbr::wireframe::{Wireframe, WireframePlugin},
    color::palettes::css,
    prelude::*
};
use bevy_procedural_track::{profile::{EpBox}, track_mesh };
fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            WireframePlugin::default()
        ))
        .add_systems(Startup, (env, create_track).chain())
        .run();

}

// ---

#[derive(Component)]
struct Socket{
    position: Vec3,
    normal: Vec3
}

fn env(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    cmd.spawn((
        DirectionalLight{
            illuminance: 18e2,
            color: Color::WHITE,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        FreeCamera::default(),
        Transform::from_xyz(20., 5., 4.).looking_to(-Vec3::X , Vec3::Y),
        AmbientLight {brightness: 100., ..default()},
    ));

    let position = vec3(0., 8., 10.);
    let dimension = vec3(2., 0.5, 4.);

    // For example, here's a cuboid. Let's add a component to it that contains information about the position and normal of the connection point.
    cmd.spawn((
        Transform::from_translation(position),
        Mesh3d(meshes.add(Cuboid::from_size(dimension))),
        MeshMaterial3d(materials.add(Color::from(css::LIGHT_GOLDENROD_YELLOW).with_alpha(0.5))),
        Socket {
            position: position - Vec3::Z * 0.5 * dimension.z,
            normal: -Vec3::Z
        }
    ));
}

// ---

fn create_track(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ct: Single<&Socket>
) {
    let socket = ct.into_inner();
    let from = vec3(0., 0., -4.);
    let to = socket.position;
    let to_socket = to - from;

    // How many control points do we need?
    let control_points_count = 5;

    // we'll make the required number of dots in the most primitive way
    let step = to_socket / (control_points_count - 1) as f32;
    let points = (0 ..control_points_count).map(| i | {
        if i == control_points_count -  2 {
            // to give a nice curve, we'll move one point
            to + socket.normal * 2.
        } else {
            from + step * i as f32
        }
    }) .collect::<Vec<_>>();


    let sub_div = 18;
    let weights = vec![1.0;control_points_count];
    let knots = CubicNurbs::<Vec3>::open_uniform_knots(control_points_count).unwrap();

    // We will generate a spline that will touch the start and end control points.
    let spline = CubicNurbs::new(points, Some(weights), Some(knots))
        .expect("NURBS construction failed!")
        .to_curve()
        .unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    // Now we'll generate a mesh for the track. It's important for us that the normal of its end matches the normal of the connection point.
    let mesh = track_mesh(&points, EpBox{half_width: 1.,half_height: 0.25}, None, Some(-socket.normal));
    let mesh = meshes.add(mesh);
    let mesh_mat = materials.add(Color::from(css::ROYAL_BLUE).with_alpha(0.5));

    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mesh_mat.clone()),
    ));

}
