use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, color::palettes::css,  pbr::wireframe::{Wireframe, WireframePlugin}, prelude::*
};
use bevy_procedural_track::{track_mesh, profile::{EpFlat, EpBox, EpSquareChannel} };
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
            illuminance: 18e2,
            color: Color::WHITE,
            // shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        FreeCamera::default(),
        Transform::from_xyz(2., 10., -4.).looking_at(Vec3::ZERO.with_z(2.)  , Vec3::Y),
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

    let points = (0 .. 10).map(| i | vec3(0., 0., i as f32)).collect::<Vec<_>>();
    let sub_div = points.len();


    // let variation = 50.;
    // let min_segment_len = 20.;
    // let sub_div = 120;

    // let mut points = RandomLoop::generate(12, vec3(100., 0., 100.));
    // RandomLoop::vary(&mut points, variation );
    // RandomLoop::smooth_out(&mut points, 120f32.to_radians(), min_segment_len);

    let spline = CubicBSpline::new(points).to_curve().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    // let mesh = track_mesh(&points, EpFlat{half_width: 1.}, false);
    let mesh = track_mesh(&points, EpBox{half_width: 1., half_height: 0.25}, false);

    let mesh = meshes.add(mesh);
    // let mesh_mat = materials.add(Color::from(css::ROYAL_BLUE));

    let mesh_mat = materials.add(StandardMaterial {
        base_color_texture: Some(assets.load("textures/road2.png")),
        // base_color: Color::from(css::GOLDENROD),
        ..default()
    });

    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mesh_mat.clone()),
        Wireframe
    ));

    // cmd.spawn((
    //     Mesh3d(meshes.add(Sphere::new(1.))),
    //     MeshMaterial3d(mesh_mat.clone()),
    // ));

}
