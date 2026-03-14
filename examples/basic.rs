use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, color::palettes::css, math::VectorSpace, pbr::wireframe::{Wireframe, WireframePlugin}, prelude::*
};
use bevy_procedural_track::{track_mesh, profile::{EpFlat, EpBox, EpSquareChannel} };
use bevy_random_loop::RandomLoop;
fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            WireframePlugin::default()
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
            illuminance: 8e2,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        FreeCamera::default(),
        Transform::from_xyz(6., 300., -4.).looking_at(Vec3::ZERO , Vec3::Y),
        AmbientLight {
            brightness: 500.,
            ..default()
        }
    ));
}

// ---

fn create_track(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    let variation = 50.;
    let min_segment_len = 20.;
    let sub_div = 120;

    // --- Flat Profile

    let mut points = RandomLoop::generate(12, vec3(100., 0., 150.));
    RandomLoop::vary(&mut points, variation );
    RandomLoop::smooth_out(&mut points , 120f32.to_radians(), min_segment_len);

    let spline = CubicBSpline::new(points).to_curve_cyclic().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, -v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;
    let mesh = track_mesh(&points, EpFlat{half_width: 4.});
    let mesh = meshes.add(mesh);
    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::from(css::ROYAL_BLUE))),
    ));

    // --- Box Profile

    let mut points = RandomLoop::generate(22, vec3(100., 0., 100.));
    RandomLoop::vary(&mut points, variation);
    RandomLoop::smooth_out(&mut points , 110f32.to_radians(), min_segment_len);
    let spline = CubicBSpline::new(points).to_curve_cyclic().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, -v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;
    let mesh = track_mesh(&points, EpBox{half_width: 4., half_height: 0.5});
    let mesh = meshes.add(mesh);
    cmd.spawn((
        Transform::from_translation(Vec3::ZERO.with_y(10.)),
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::from(css::REBECCA_PURPLE))),
    ));

    // -- SquareChannel Profile

    let mut points = RandomLoop::generate(22, vec3(80., 0., 100.));
    RandomLoop::vary(&mut points, variation);
    RandomLoop::smooth_out(&mut points , 110f32.to_radians(), min_segment_len);
    let spline = CubicBSpline::new(points).to_curve_cyclic().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, -v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;
    let mesh = track_mesh(&points, EpSquareChannel{half_width: 4.,  height: 2., depth: 1.8, border_width: 0.5});
    let mesh = meshes.add(mesh);
    cmd.spawn((
        Transform::from_translation(Vec3::ZERO.with_y(20.)),
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::from(css::SEA_GREEN))),
    ));
}
