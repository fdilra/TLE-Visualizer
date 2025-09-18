use anyhow::{Context, Result};
use three_d::*;

use crate::propagator::PropagationResult;

// TODO: draw propagated TLE trajectories 
pub fn plot_tles(results: &Vec<PropagationResult>) -> Result<()> {
    let (window, context) = init_window()?;
    let (camera, orbit_control) = init_camera(&window);
    let axes = Axes::new(&context, 0.05, 3.0); // XYZ axes
    let earth_model = load_earth_model(&context)?;
    let ambient_light = AmbientLight::new(&context, 0.4, Srgba::WHITE);

    render_loop(window, camera, orbit_control, earth_model, axes, ambient_light)?;

    Ok(())
}

fn init_window() -> Result<(Window, three_d::Context)> {
    let window = Window::new(WindowSettings {
        title: "TLE Visualizer".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    }).context("ERROR: Failed to open a new window")?;

    let context = window.gl();

    Ok((window, context))
}

fn init_camera(window: &Window) -> (Camera, OrbitControl) {
    // Camera looks at origin
    let camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.0, 3.0, 3.0),
        vec3(0.0, 0.0, 0.0), 
        vec3(0.0, 1.0, 0.0), 
        degrees(45.0),
        0.1,
        1000.0,
    );
    // Orbit control locked around origin
    let control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 100.0);

    return (camera, control);
}

fn load_earth_model(context: &three_d::Context) -> Result<ModelPart<PhysicalMaterial>> {
    // Load model from file
    let mut loaded = three_d_asset::io::load(&[
            "assets/earth.glb",
    ])?;

    // Earth model
    let mut cpu_model: CpuModel = loaded.deserialize("earth").unwrap();
    cpu_model
        .geometries
        .iter_mut()
        .for_each(|m| m.compute_tangents());
    let mut model = Model::<PhysicalMaterial>::new(context, &cpu_model)
        .unwrap()
        .remove(0);
    
    // Center the Earth model at the origin
    // Get the axis-aligned bounding box (AABB) from the model and its center
    let aabb = model.aabb();
    let center = aabb.center();
    // Apply tranformation to the entire model
    // TODO: investigate why it is necessary to multiply the denom by a factor of 10 to get the correct position
    model.set_transformation(Mat4::from_translation(-center/20.0));

    Ok(model)
}

pub fn render_loop(
    window: Window, 
    mut camera: Camera, 
    mut control: OrbitControl, 
    model: ModelPart<PhysicalMaterial>,
    axes: Axes,
    ambient_light: AmbientLight
) -> Result<()> {
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render(
                &camera, 
                model.into_iter()
                .chain(&axes),
                &[&ambient_light]);

        FrameOutput::default()
    });

    Ok(())
}



