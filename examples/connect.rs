// An example demonstrating the connection of a track and a cuboid

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
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
        Transform::from_xyz(20., 5., 14.).looking_to(-Vec3::X , Vec3::Y),
        AmbientLight {brightness: 100., ..default()},
    ));

    let position = vec3(0., 8., 20.);
    let dimension = vec3(2., 0.5, 4.);

    // For example, here's a cuboid. Let's add a component to it that contains information about the position and normal of the connection point.
    cmd.spawn((
        Transform::from_translation(position).with_rotation(Quat::from_rotation_x(15.0_f32.to_radians())),
        Mesh3d(meshes.add(Cuboid::from_size(dimension))),
        MeshMaterial3d(materials.add(Color::from(css::LIGHT_CORAL))),
        Socket {
            position: -Vec3::Z * 0.5 * dimension.z,
            normal: -Vec3::Z
        }
    ));
}

// ---

fn create_track(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ct: Single<(&Socket, &Transform)>
) {
    let (socket, transform) = ct.into_inner();
    let start_pos = vec3(1., 0., -4.);
    let start_velocity = Vec3::Z;
    let velocity_magnitude = 25.;

    let end_pos = transform.translation + transform.rotation.mul_vec3(socket.position);
    let end_velocity = transform.rotation.mul_vec3(-socket.normal).normalize();
    let spline = CubicHermite::new(vec![start_pos, end_pos], vec![start_velocity * velocity_magnitude, end_velocity * velocity_magnitude ]).to_curve().unwrap();

    // cmd.spawn((
    //     Mesh3d(
    //         meshes.add(Polyline3d::new(
    //             spline.iter_positions(100)
    //         )),
    //     ),
    //     MeshMaterial3d(
    //         materials.add(StandardMaterial {
    //             emissive: LinearRgba::from(Color::from(css::BLUE_VIOLET)),
    //             ..default()
    //         })
    //     )
    // ));


    let sub_div = 18;
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    // Now we'll generate a mesh for the track. It's important for us that the normal of its end matches the normal of the connection point.
    let mesh = track_mesh(&points, EpBox{half_width: 1.,half_height: 0.25}, None, Some(end_velocity));
    let mesh = meshes.add(mesh);
    let mesh_mat = materials.add(Color::from(css::ROYAL_BLUE));

    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mesh_mat.clone()),
    ));

}
