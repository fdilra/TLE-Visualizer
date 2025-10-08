use anyhow::{Context, Result};
use eframe::egui;
use egui_plot::{Line, Plot, PlotImage, PlotPoint, PlotPoints, Points}; // <-- Added Points here
use crate::propagator::PropagationResult;

/// Convert Earth-centered inertial (x,y,z) km position into (lat, lon) degrees
fn eci_to_latlon(pos: &[f64; 3]) -> (f64, f64) {
    let r = (pos[0].powi(2) + pos[1].powi(2) + pos[2].powi(2)).sqrt();
    let lat = (pos[2] / r).asin().to_degrees();
    let lon = pos[1].atan2(pos[0]).to_degrees();
    (lat, lon)
}

/// Helper function to load an image and convert it to an egui texture.
/// Loads an image and returns an egui `TextureHandle`.
fn load_background_image(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let image_path = "assets/earthtex.jpg";

    let image_result = image::open(image_path);
    let image = match image_result {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            // Log an error if the image can't be loaded.
            // The application will continue without a background.
            eprintln!("Failed to load background image '{}': {}", image_path, e);
            return None;
        }
    };

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    // Load the image as a texture and return it
    Some(ctx.load_texture(
        "world_map_texture",
        color_image,
        egui::TextureOptions::default(),
    ))
}


/// Application state for egui
struct GroundTrackApp {
    results: Vec<PropagationResult>,

    // Using Option<> allows the app to run even if the image fails to load
    background_texture: Option<egui::TextureHandle>,
}

// TODO: improve readability
impl eframe::App for GroundTrackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TLE Visualizer - Ground Track");
            Plot::new("ground_track_plot")
                .data_aspect(1.5) // longitude/latitude aspect ratio
                // Set the plot bounds to match the map coordinates
                // Longitude: -180 to 180, Latitude: -90 to 90
                .min_size([0.0, 0.0].into()) // Ensures the plot is visible
                .view_aspect(2.0) // TODO: find the optimal aspect ratio of the plot region
                .show(ui, |plot_ui| {
                    if let Some(texture) = &self.background_texture {
                        let image = PlotImage::new(
                            texture.id(),
                            PlotPoint::new(0.0, 0.0), // Center of the map (lon=0, lat=0)
                            [360.0, 180.0], // Size of the map in plot coordinates (360 degrees lon, 180 degrees lat)
                        );
                        plot_ui.image(image);
                    }

                    let colors = [
                        egui::Color32::RED,
                        egui::Color32::GREEN,
                        egui::Color32::BLUE,
                        egui::Color32::YELLOW,
                        egui::Color32::WHITE,
                    ];

                    for (i, result) in self.results.iter().enumerate() {
                        let color = colors[i % colors.len()];

                        let points: PlotPoints = result
                            .positions
                            .iter()
                            .map(|p| {
                                let (lat, lon) = eci_to_latlon(p);
                                [lon, lat] // x = longitude, y = latitude
                            })
                            .collect();

                        let line = Line::new(points).color(color).width(2.0);
                        plot_ui.line(line);

                        // Highlight first point of the trajectory
                        // Check if there are any positions to avoid a panic
                        if let Some(first_pos) = result.positions.first() {
                            let (lat, lon) = eci_to_latlon(first_pos);
                            
                            // Create a 'Points' item with a single point
                            let start_point = Points::new([lon, lat])
                                .radius(5.0)   
                                .color(color); 

                            plot_ui.points(start_point);
                        }
                    }
                });
        });
    }
}

/// Launch a 2D egui app for ground tracks.
pub fn plot_2d(results: &Vec<PropagationResult>) -> Result<()> {
    let native_options = eframe::NativeOptions::default();

    let results_clone = results.clone();

    let result = eframe::run_native(
        "Ground Tracks",
        native_options,
        Box::new(|cc| {
            // Load the background image once during app creation
            let background_texture = load_background_image(&cc.egui_ctx);
            // Create the application state
            Box::new(GroundTrackApp {
                results: results_clone,
                background_texture,
            })
        }),
    ); // This should have '?' but adding it causes an error 

    Ok(())
}