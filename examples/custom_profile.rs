// An example showing the creation of a route with a custom triangular cross-section profile
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, color::palettes::css,  pbr::wireframe::{Wireframe, WireframePlugin}, prelude::*
};
use bevy_procedural_track::{ElementProfile, track_mesh};
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

// New custom profile
pub struct EpTriangularDitch {
    pub half_width: f32,
    pub height: f32
}
// Need  implement ElementProfile for new profile
impl ElementProfile for EpTriangularDitch {
    fn cut(&self,  base: &Vec3, tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3> {
        let normal  = Self::normal(tangent, bnormal);
        vec![
            base - bnormal * self.half_width + normal * self.height,
            *base,
            base + bnormal * self.half_width  +  normal * self.height,
        ]
    }

    fn build(&self, prev: &Vec<Vec3>, current: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>, uvs: &mut Vec<[f32;2]>) {
        let j = verts.len() as u32;
        verts.extend(vec![
            prev[1], current[1], prev[0], current[0],
            current[1], prev[1], current[2], prev[2],
            ].iter().map(Vec3::to_array)
        );
        idxs.extend_from_slice(&[
            j, j + 1, j + 2,
            j + 2, j + 1, j + 3,
            j + 4, j + 5, j + 6,
            j + 5, j + 7, j + 6,
        ]);

        uvs.extend_from_slice(&[
            [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1., 0.],
            [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1., 0.],
        ]);

    }

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
        Transform::from_xyz(2., 5., -4.).looking_at(Vec3::ZERO , Vec3::Y),
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
    // assets: ResMut<AssetServer>
) {

    let sub_div = 120;
    let points = (0 .. 20).map(| i | vec3(i as f32, 0., (i as f32).to_degrees().sin() )).collect::<Vec<_>>();

    let spline = CubicBSpline::new(points).to_curve().unwrap();
    let points = spline.iter_positions(sub_div)
        .zip(spline.iter_velocities(sub_div))
        .map(| ( p, v ) | ( p, v.normalize().cross(Vec3::Y).normalize() ))
        .collect::<Vec<_>>()
    ;

    let mesh = track_mesh(&points, EpTriangularDitch{half_width: 0.2, height: 0.1}, false);
    let mesh = meshes.add(mesh);

    let mesh_mat = materials.add(StandardMaterial {
        // base_color_texture: Some(assets.load("textures/road_flat.png")),
        base_color: Color::from(css::ROYAL_BLUE),
        ..default()
    });

    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mesh_mat.clone()),
        // Wireframe
    ));
}
