use avian3d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::{camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},  prelude::*, pbr::wireframe::WireframePlugin};

mod track;
mod profile;
mod functions;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        FreeCameraPlugin,
        PhysicsDebugPlugin::default(),
        PhysicsPlugins::default(),
        WireframePlugin::default()
    ))
    .add_plugins(track::TrackPlugin)
    .add_systems(Startup, startup)
    .run();
}

// ---

fn startup(
    mut cmd: Commands,

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
        Transform::from_xyz(6., 80., 4.).looking_at(Vec3::ZERO , Vec3::Y),
        AmbientLight {
            brightness: 500.,
            ..default()
        }
    ));
}
