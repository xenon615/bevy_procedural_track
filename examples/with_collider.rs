use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    color::palettes::css,
    prelude::*
};
use bevy_procedural_track::{track_mesh, profile::EpSquareChannel };
use avian3d::prelude::*;
fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin

        ))

        .add_systems(Startup, startup)
        .run();

}

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn_scene(bsn! {
        DirectionalLight{
            illuminance: 18e2,
            color: Color::WHITE,
            shadow_maps_enabled: true
        }
        template_value(Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y))
    });

    cmd.spawn_scene(bsn!{
        Camera3d::default()
        Camera::default()
        FreeCamera::default()
        template_value(Transform::from_xyz(30., 2., -2.).looking_at(Vec3::ZERO , Vec3::Y))
        AmbientLight {brightness: 100.,}
    });

    cmd.spawn_scene(bsn! {
       Transform::from_xyz(8., 6., 0.)
       Mesh3d(asset_value(Sphere::new(1.)))
       MeshMaterial3d<StandardMaterial>(asset_value(Color::from(css::LIGHT_GOLDENROD_YELLOW)))
       Collider::sphere(1.)
       template_value(RigidBody::Dynamic)
    });


    let sub_div = 20;
    let points = (0 .. 20).map(| i | vec3(i as f32, -0.1 * i as f32 , (i as f32).to_degrees().sin() * 0.5 )).collect::<Vec<_>>();

    let spline = CubicBSpline::new(points).to_curve().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    let mesh = track_mesh(&points, EpSquareChannel{half_width: 2., height: 1., depth: 0.5, border_width: 0.2}, false);

    cmd.spawn_scene(bsn!{
        Mesh3d(asset_value(mesh))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::from(css::ROYAL_BLUE)))
        template_value(ColliderConstructor::TrimeshFromMeshWithConfig(TrimeshFlags::FIX_INTERNAL_EDGES))
        template_value(RigidBody::Static)
        template_value(Restitution::new(0.5).with_combine_rule(CoefficientCombine::Max))
    });
}
