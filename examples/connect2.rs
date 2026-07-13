
// how to connect track to track

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

fn env(
    mut cmd: Commands,
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
        Transform::from_xyz(-10., 5., 5.).looking_at(Vec3::ZERO , Vec3::Y),
        AmbientLight {brightness: 500., ..default()},
    ));
}

// ---

fn create_track(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // materials for both tracks
    let track1_mat = materials.add(Color::from(css::ORANGE_RED));
    let track2_mat = materials.add(Color::from(css::CADET_BLUE));

    // let it be 15 control points
    let control_points_count = 15;

    //  function for points calculation , leit be sinus ? for sinplicity
    let ss = | d | {
        let r = (d as f32).to_radians() * 10.;
        vec3(0., r.sin() * 2., r)
    };

    // calculate points for both tracks
    let points1 = (0..15).map(ss).collect::<Vec<_>>();
    let points2 = (14..29).map(ss).collect::<Vec<_>>();

    // subdivs count for splain , more value - more smooth
    let sub_div = 8;

    // waights and knots for NURBS
    //  accoding to docs we could use use default values
    // like CubicNurbs::new(points, None, None))
    // but compiler complains about God knows what
    // so  lets pass  them  by explicit way
    // in this case these paramerers can  equal for  both splines

    let weights = vec![1.0;control_points_count];
    let knots = CubicNurbs::<Vec3>::open_uniform_knots(control_points_count).unwrap();

    let spline1 = CubicNurbs::new(points1, Some(weights.clone()), Some(knots.clone()))
        .expect("NURBS construction failed!")
        .to_curve()
        .unwrap();

    let spline2 = CubicNurbs::new(points2, Some(weights), Some(knots))
        .expect("NURBS construction failed!")
        .to_curve()
        .unwrap();

    let points1 = spline1.iter_positions(sub_div)
        .zip(spline1.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    let points2 = spline2.iter_positions(sub_div)
        .zip(spline2.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    // lets make meshes for both tracks
    // two last nonrequired params are nornals to first cut of profile and to last cut  respectively.
    //  in our case we dont care about first cut of first track and last cut of last track
    //  so we need to specify only 2 of theese params
    // for simplicity let's make a horizontal cuе in order to they suited each other

    let mesh1 = track_mesh(&points1, EpBox{half_width: 1.,half_height: 0.25}, None, Some(-Vec3::Y));
    let mesh2 = track_mesh(&points2, EpBox{half_width: 1.,half_height: 0.25}, Some(-Vec3::Y), None);

    let mesh1 = meshes.add(mesh1);
    let mesh2 = meshes.add(mesh2);

    cmd.spawn((
        Mesh3d(mesh1.clone()),
        MeshMaterial3d(track1_mat.clone()),
    ));
    cmd.spawn((
        Mesh3d(mesh2.clone()),
        MeshMaterial3d(track2_mat.clone()),
    ));

}
