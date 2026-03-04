use bevy::{camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},  prelude::*, pbr::wireframe::WireframePlugin};
use avian3d::prelude::*;
fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            PhysicsDebugPlugin::default(),
            PhysicsPlugins::default(),
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
        Transform::from_xyz(6., 8., 4.).looking_at(Vec3::ZERO , Vec3::Y),
        AmbientLight {
            brightness: 500.,
            ..default()
        }
    ));
}

fn create_track(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::WHITE))
    ));
}
