//An example demonstrating the use of texture

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},  prelude::*
};
use bevy_procedural_track::{track_mesh, profile::EpSquareChannel };
use bevy_random_loop::RandomLoop;

fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set (
                AssetPlugin {
                    file_path: "examples/assets".to_string(),
                    ..default()
                }
            ),
            FreeCameraPlugin,
        ))
        .add_systems(Startup, (startup, create_track))
        .run();

}

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        DirectionalLight{
            illuminance: 18e2,
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        FreeCamera::default(),
        Transform::from_xyz(50., 200., -4.).looking_at(Vec3::ZERO.with_z(2.)  , Vec3::Y),
        AmbientLight {
            brightness: 100.,
            ..default()
        }
    ));
}

// ---

fn create_track(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>
) {

    let variation = 50.;
    let min_segment_len = 10.;
    let sub_div = 120;

    let mut points = RandomLoop::generate(12, vec3(100., 10., 100.));
    RandomLoop::vary(&mut points, variation );
    RandomLoop::smooth_out(&mut points, 90f32.to_radians(), min_segment_len);

    let spline = CubicBSpline::new(points).to_curve_cyclic().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    let mesh = track_mesh(&points, EpSquareChannel{half_width: 1., height: 0.5, depth: 0.35, border_width: 0.2}, true);

    //You may want to use your own UV according to your texture.
    // something like this, for example
    /*
    let mut mesh = track_mesh(&points, EpSquareChannel{half_width: 1., height: 0.5, depth: 0.35, border_width: 0.2}, false);
    mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
    let count_vertices = mesh.count_vertices();
    let uvs = (0 .. count_vertices / 32).flat_map(| _ | [
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // bottom
                [1.0, 0.5], [0.0, 0.5], [1.0, 0.0], [0., 0.], // top
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right border
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left border
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right inner
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left inner
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right outer
                [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left outer

        ]).collect::<Vec<_>>();
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    */


    let mesh = meshes.add(mesh);
    let mesh_mat = materials.add(StandardMaterial {
        base_color_texture: Some(assets.load("textures/road_box.png")),
        ..default()
    });

    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mesh_mat.clone()),
    ));

}
