extern crate kiss3d;
extern crate nalgebra as na;

use anyhow::Result;
use kiss3d::camera::Camera;
use kiss3d::{camera::ArcBall, scene::SceneNode};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3, OPoint, Const};
use std::{f32::consts, path::Path};

use crate::propagator::PropagationResult;

/// Scale factor to convert orbital positions (in km) to reasonable units
const KM_TO_UNIT_SCALE: f64 = 6371.137 / 5.0;

/// Render the Earth-centered trajectories as line segments around a 3D model of the Earth
pub fn plot_3d(results: &Vec<PropagationResult>) -> Result<()> {
    let mut window = Window::new("TLE Visualizer");
    window.set_light(Light::StickToCamera);

    let mut camera = setup_camera();
    let mut skybox = setup_skybox(&mut window);
    let mut earth  = setup_earth(&mut window);

    let colors = define_colors();
    
    // Render loop
    while window.render_with_camera(&mut camera) {
        camera.look_at(camera.eye(), Point3::origin()); // Reset camera pointing to origin
        window.set_line_width(2.0);

        // Move skybox
        skybox.set_local_translation(camera.at().into());

        draw_axes(&mut window, &colors);
        draw_trajectories(&mut window, results, &colors);
    }

    Ok(())
}

/// Create an ArcBall camera to orbit around the origin
fn setup_camera() -> ArcBall {
    let eye = Point3::new(10.0, 10.0, 10.0);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);
    camera.set_dist_step(1.005);

    return camera;
}

/// Create a skybox and load its texture.
/// Note: the stars are only for eye candy and the position of the sky
/// relative to the Earth is not accurate.
fn setup_skybox(window: &mut Window) -> SceneNode {
    let mut skybox = window.add_sphere(1000.0);
    let skybox_texture_path = Path::new("assets/stars.jpg");
    skybox.set_texture_from_file(skybox_texture_path, "stars");
    skybox.enable_backface_culling(false);

    return skybox;
}

/// Create a sphere, apply the Earth texture, and rotate it to fit an Earth fixed coord system
fn setup_earth(window: &mut Window) -> SceneNode {
    // TODO: fix texture/model alignment with coord system
    let mut earth = window.add_sphere(5.0);
    let texture_path = Path::new("assets/earthtex.jpg");
    earth.set_texture_from_file(texture_path, "earth");

    let rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), consts::PI);
    earth.append_rotation_wrt_center(&rotation);
    let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), consts::FRAC_PI_2);
    // earth.append_rotation_wrt_center(&rotation);

    return earth;
}

/// Define some colors to cycle through in the case of satellite groups
fn define_colors() -> [OPoint<f32, Const<3>>; 7] {
    let colors = [
        Point3::new(1.0, 0.0, 0.0), // red
        Point3::new(0.0, 1.0, 0.0), // green
        Point3::new(0.0, 0.0, 1.0), // blue
        Point3::new(1.0, 1.0, 0.0), 
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0), // white
    ];

    return colors;
}

fn draw_axes(window: &mut Window, colors: &[OPoint<f32, Const<3>>; 7]) {
    // Define axes points
    let x_axis_pt = Point3::new(-10.0, 0.0, 0.0);
    let y_axis_pt = Point3::new(0.0, 10.0, 0.0);
    let z_axis_pt = Point3::new(0.0, 0.0, -10.0); 

    // Draw axes
    window.draw_line(&Point3::origin(), &x_axis_pt, &colors[0]);
    window.draw_line(&Point3::origin(), &y_axis_pt, &colors[1]);
    window.draw_line(&Point3::origin(), &z_axis_pt, &colors[2]);
}

fn draw_trajectories(
    window: &mut Window, 
    results: &Vec<PropagationResult>,
    colors: &[OPoint<f32, Const<3>>; 7]
) {
    for (i, result) in results.iter().enumerate() {
        let color = colors[i % colors.len()];

        // Convert propagated positions into scaled 3D points
        let positions: Vec<Point3<f32>> = result
            .positions
            .iter()
            .map(|p| {
                Point3::new(
                    (-p[0] / KM_TO_UNIT_SCALE) as f32,
                    (p[2] / KM_TO_UNIT_SCALE) as f32,
                    (p[1] / KM_TO_UNIT_SCALE) as f32,
                )
            })
            .collect();

        // Draw each consecutive segment
        for w in positions.windows(2) {
            if let [p1, p2] = &w {
                window.draw_line(p1, p2, &color);
            }
        }
    }
}