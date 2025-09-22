extern crate kiss3d;
extern crate nalgebra as na;

use anyhow::Result;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3};
use std::{f32::consts, path::Path};

use crate::propagator::PropagationResult;

/// Scale factor to convert orbital positions (in km) to reasonable units
const KM_TO_UNIT_SCALE: f64 = 6371.137 / 5.0;

/// Render the Earth-centered trajectories as line segments
pub fn plot_tles(results: &Vec<PropagationResult>) -> Result<()> {
    let mut window = Window::new("TLE Visualizer");
    window.set_light(Light::StickToCamera);

    // Create an ArcBall camera to orbit around the origin.
    let eye = Point3::new(10.0, 10.0, 10.0);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);
    camera.set_dist_step(1.005);

    // Add a sphere to represent the Earth and rotate it to fit an Earth fixed coord system
    let mut earth = window.add_sphere(5.0);
    let texture_path = Path::new("assets/earthtex.jpg");
    earth.set_texture_from_file(texture_path, "earth");
    let rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), consts::PI);
    earth.append_rotation_wrt_center(&rotation);
    let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), consts::FRAC_PI_2);
    earth.append_rotation_wrt_center(&rotation);

    // Define some colors to cycle through for multiple trajectories
    let colors = [
        Point3::new(1.0, 0.0, 0.0), // red
        Point3::new(0.0, 1.0, 0.0), // green
        Point3::new(0.0, 0.0, 1.0), // blue
        Point3::new(1.0, 1.0, 0.0), 
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0), // white
    ];

    // Define axes points
    let x_axis_pt = Point3::new(10.0, 0.0, 0.0);
    let y_axis_pt = Point3::new(0.0, 10.0, 0.0);
    let z_axis_pt = Point3::new(0.0, 0.0, 10.0); 

    // Render loop
    while window.render_with_camera(&mut camera) {
        window.set_line_width(2.0);

        // Draw axes
        window.draw_line(&at, &x_axis_pt, &colors[0]);
        window.draw_line(&at, &y_axis_pt, &colors[1]);
        window.draw_line(&at, &z_axis_pt, &colors[2]);

        // Draw trajectories
        for (i, result) in results.iter().enumerate() {
            let color = colors[i % colors.len()];

            // Convert propagated positions into scaled 3D points
            let positions: Vec<Point3<f32>> = result
                .positions
                .iter()
                .map(|p| {
                    Point3::new(
                        (p[0] / KM_TO_UNIT_SCALE) as f32,
                        (p[1] / KM_TO_UNIT_SCALE) as f32,
                        (p[2] / KM_TO_UNIT_SCALE) as f32,
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

    Ok(())
}