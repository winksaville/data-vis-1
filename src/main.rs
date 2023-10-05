//! Simple 3d plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://github.com/plotters-rs/plotters/blob/master/plotters/examples/3d-plot.rs)

use std::time::Duration;

use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::{Chart, MouseConfig};
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        env!("APP_NAME"),
        native_options,
        Box::new(|cc| Box::new(Chart3d::new(cc))),
    )
    .unwrap();
}

struct Chart3d {
    chart: Chart<()>,
}

impl Chart3d {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathring as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        // Create a new 3d chart with all mouse controls enabled and the chart slightly angled
        let chart = Chart::new(())
            .mouse(MouseConfig::enabled())
            .pitch(0.3)
            .yaw(0.7)
            .builder_cb(Box::new(|area, transform, _d| {
                // Build a chart like you would in any other plotter chart.
                // The drawing area and projection transformations are provided
                // by the callback, but otherwise everything else is the same.
                let x_axis = (-3.0..3.0).step(0.1);
                let z_axis = (-3.0..3.0).step(0.1);

                // Define a chart with a caption and 3d cartesian coordinate system
                let mut chart = ChartBuilder::on(area)
                    .caption("Data visualization 1", (FontFamily::SansSerif, 20))
                    .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
                    .unwrap();

                // Position the camera
                chart.with_projection(|mut pb| {
                    pb.yaw = transform.yaw;
                    pb.pitch = transform.pitch;
                    pb.scale = 0.7; // Set scale to 0.7 to avoid artifacts caused by plotter's renderer
                    pb.into_matrix()
                });

                // Draw a axis, grid and grid labels
                chart
                    .configure_axes()
                    .light_grid_style(BLACK.mix(0.15))
                    .max_light_lines(3)
                    .draw()
                    .unwrap();

                // Draw a SurfaceSeries in BLUE and it's label is "Surface"
                chart
                    .draw_series(
                        SurfaceSeries::xoz(
                            (-30..30).map(|f| f as f64 / 10.0),
                            (-30..30).map(|f| f as f64 / 10.0),
                            |x, z| (x * x + z * z).cos(),
                        )
                        .style(BLUE.mix(0.2).filled()),
                    )
                    .unwrap()
                    .label("Surface")
                    .legend(|(x, y)| {
                        Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
                    });

                // Draw a LineSeries in BLACK and it's label is "Line"
                chart
                    .draw_series(LineSeries::new(
                        (-100..100)
                            .map(|y| y as f64 / 40.0)
                            .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                        &BLACK,
                    ))
                    .unwrap()
                    .label("Line")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

                // Draw the legend for all elements in the chart
                chart
                    .configure_series_labels()
                    .border_style(BLACK)
                    .draw()
                    .unwrap();
            }));

        Self { chart }
    }
}

impl eframe::App for Chart3d {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.chart.draw(ui);
        });

        // Limit framerate to 100fps
        std::thread::sleep(Duration::from_millis(10));
        ctx.request_repaint();
    }
}
