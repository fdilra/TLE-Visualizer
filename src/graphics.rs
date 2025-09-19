use anyhow::{Context, Result};
use three_d::*;

use crate::propagator::PropagationResult;

// A scale factor to convert orbital positions (in km) to the scene's units.
const KM_TO_UNIT_SCALE: f64 = 6371.137;
// The radius of the cylinders used to draw the trajectory.
const TRAJECTORY_RADIUS: f32 = 0.002;

/// Renders the Earth and the provided satellite trajectories using instanced cylinders.
pub fn plot_tles(results: &Vec<PropagationResult>) -> Result<()> {
    let (window, context) = init_window()?;
    let (camera, orbit_control) = init_camera(&window);
    let axes = Axes::new(&context, 0.05, 3.0);
    let earth_model = load_earth_model(&context)?;
    let ambient_light = AmbientLight::new(&context, 0.5, Srgba::WHITE);
    let trajectories = generate_trajectories(&context, results)?;

    render_loop(
        window,
        camera,
        orbit_control,
        earth_model,
        axes,
        ambient_light,
        trajectories,
    )?;

    Ok(())
}

/// Generate the models for the trajectories out of cylinders
fn generate_trajectories(context: &three_d::Context, results: &Vec<PropagationResult>) -> Result<Vec<Gm<InstancedMesh, PhysicalMaterial>>> {
    // Create a base cylinder mesh to represent one segment of a trajectory.
    let mut cylinder_cpu_mesh = CpuMesh::cylinder(3);
    cylinder_cpu_mesh
        .transform(Mat4::from_nonuniform_scale(
            1.0, // Length is handled by edge_transform
            TRAJECTORY_RADIUS,
            TRAJECTORY_RADIUS,
        ))?;

    // Define a list of colors for the trajectories
    let colors = [
        Srgba::RED, Srgba::GREEN, Srgba::BLUE, Srgba::WHITE
    ];

    // For each PropagationResult, create an InstancedMesh of cylinders.
    let trajectories: Vec<Gm<InstancedMesh, PhysicalMaterial>> = results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            // Convert propagated positions to scaled Vec3 points
            let positions: Vec<Vec3> = result
                .positions
                .iter()
                .map(|p| {
                    vec3(
                        (p[0] / KM_TO_UNIT_SCALE) as f32,
                        (p[1] / KM_TO_UNIT_SCALE) as f32,
                        (p[2] / KM_TO_UNIT_SCALE) as f32,
                    )
                })
                .collect();

            // Generate the transformations for each cylinder segment
            let instances = create_trajectory_transformations(&positions);

            // Create a colored material for this trajectory
            let material = PhysicalMaterial::new_opaque(
                context,
                &CpuMaterial {
                    albedo: colors[i % colors.len()],
                    ..Default::default()
                },
            );

            // Create the renderable object
            Gm::new(
                InstancedMesh::new(context, &instances, &cylinder_cpu_mesh),
                material,
            )
        })
        .collect();

    Ok(trajectories)
}

/// Generates the instance transformations for cylinder segments connecting a list of points.
fn create_trajectory_transformations(positions: &[Vec3]) -> Instances {
    let transformations = positions
        .windows(2) // Create a sliding window of 2 points, i.e., [p1, p2], [p2, p3], ...
        .map(|points| edge_transform(points[0], points[1])) // Create a transform for the segment
        .collect();

    Instances {
        transformations,
        ..Default::default()
    }
}

/// Calculates the transformation matrix to scale, rotate, and position a cylinder to connect point p1 and p2
fn edge_transform(p1: Vec3, p2: Vec3) -> Mat4 {
    let length = (p1 - p2).magnitude();
    let rotation = Quat::from_arc(vec3(1.0, 0.0, 0.0), (p2 - p1).normalize(), None);
    
    Mat4::from_translation(p1) * Mat4::from(rotation) * Mat4::from_nonuniform_scale(length, 1.0, 1.0)
}

fn init_window() -> Result<(Window, three_d::Context)> {
    let window = Window::new(WindowSettings {
        title: "TLE Visualizer".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .context("ERROR: Failed to open a new window")?;
    let context = window.gl();
    Ok((window, context))
}

fn init_camera(window: &Window) -> (Camera, OrbitControl) {
    let camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.0, 3.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let control = OrbitControl::new(camera.target(), 1.0, 100.0);
    (camera, control)
}

fn load_earth_model(context: &three_d::Context) -> Result<Model<PhysicalMaterial>> {
    let mut loaded = three_d_asset::io::load(&["assets/earth.glb"])?;
    let cpu_model: CpuModel = loaded.deserialize("earth").unwrap();
    let mut model = Model::<PhysicalMaterial>::new(context, &cpu_model).unwrap();
    let aabb = model.iter().next().unwrap().aabb();

    // Define the scale
    let scale = 2.0 / aabb.max().y;
    // Rotate model clockwise
    let rotation = Mat4::from_angle_y(degrees(-90.0));
    // Combine the transformations (scale first, then rotate)
    let transformation = rotation * Mat4::from_scale(scale);
    // Apply the combined transformation
    model.iter_mut().for_each(|m| m.set_transformation(transformation));
    
    Ok(model)
}

pub fn render_loop(
    window: Window,
    mut camera: Camera,
    mut control: OrbitControl,
    model: Model<PhysicalMaterial>,
    axes: Axes,
    ambient_light: AmbientLight,
    trajectories: Vec<Gm<InstancedMesh, PhysicalMaterial>>, 
) -> Result<()> {
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let objects = model
            .iter()
            .map(|m| m as &dyn Object)
            .chain(trajectories.iter().map(|t| t as &dyn Object))
            .chain(std::iter::once(&axes as &dyn Object));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.05, 0.05, 0.1, 1.0, 1.0))
            .render(
                &camera,
                objects,
                &[&ambient_light],
            );

        FrameOutput::default()
    });
    
    Ok(())
}