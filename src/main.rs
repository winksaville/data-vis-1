use std::time::Duration;

use data_vis_1::{predict_y, sanco2_coeffs, sanco2_points};
use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::{Chart, MouseConfig};
use plotters::prelude::*;

// Set to true to show the sanco2_points
const SHOW_SANCO2_POINTS: bool = false;

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
                // The drawing area and projection trans5ormations are provided
                // by the callback, but otherwise everything else is the same.
                let x_axis = (-15.0..110.0).step(5.0);
                let z_axis = (140.0..175.0).step(5.0);
                let y_axis = (0.0..6.0).step(0.1);

                // Define a chart with a caption and 3d cartesian coordinate system
                let mut chart = ChartBuilder::on(area)
                    .caption("Data visualization 1", (FontFamily::SansSerif, 20))
                    .build_cartesian_3d(x_axis, y_axis, z_axis)
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
                    .x_formatter(&|x| format!("x={x:0.1}"))
                    .y_formatter(&|y| format!("y={y:0.1}"))
                    .z_formatter(&|z| format!("z={z:0.1}"))
                    .light_grid_style(BLACK.mix(0.15))
                    .max_light_lines(3)
                    .draw()
                    .unwrap();

                let coeffs = sanco2_coeffs();

                // Draw a SurfaceSeries in RED and it's label is "Scan02"
                chart
                    .draw_series(
                        SurfaceSeries::xoz(
                            (-15..=110).map(|f| {
                                #[allow(clippy::let_and_return)]
                                let a = f as f64;
                                //println!("sanco2 a: {a:0.2?}");
                                a
                            }),
                            (140..=175).map(|f| {
                                #[allow(clippy::let_and_return)]
                                let b = f as f64;
                                //println!("sanco2 b: {b:0.2?}");
                                b
                            }),
                            |x, z| {
                                #[allow(clippy::let_and_return)]
                                let y = predict_y(x, z, &coeffs);
                                //println!("sanco2: x:{x:0.2} z:{z:0.2} = y:{y:0.2}");
                                y
                            },
                        )
                        .style(RED.mix(0.1).filled()),
                    )
                    .unwrap()
                    .label("Sanc02")
                    .legend(|(x, y)| {
                        Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], RED.mix(0.8).filled())
                    });

                if SHOW_SANCO2_POINTS {
                    // Draw a Scatter in Blue and it's label is "Scan02 Points"
                    chart
                        .draw_series(
                            SurfaceSeries::xoz(
                                sanco2_points().iter().map(|point| {
                                    //println!("sanco2_points a: x:{:0.2?}", point.x);
                                    point.x
                                }),
                                sanco2_points().iter().map(|point| {
                                    //println!("sanco2_points b: z:{:0.2?}", point.z);
                                    point.z
                                }),
                                |x, z| {
                                    #[allow(clippy::let_and_return)]
                                    let y = predict_y(x, z, &coeffs);
                                    //println!("sanco2_points: x:{x:0.2} z:{z:0.2} = y:{y:0.2}");
                                    y
                                },
                            )
                            .style(BLUE.mix(0.01).filled()),
                        )
                        .unwrap()
                        .label("Sanc02 Points")
                        .legend(|(x, y)| {
                            Rectangle::new(
                                [(x + 5, y - 5), (x + 15, y + 5)],
                                BLUE.mix(0.8).filled(),
                            )
                        });
                }

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
        //println!("Sleeping..");
        //std::thread::sleep(Duration::from_millis(2000));
        //println!();
        std::thread::sleep(Duration::from_millis(10));
        ctx.request_repaint();
    }
}
