use eframe::egui;

pub fn overlay() -> Result<(), eframe::Error> {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_decorations(false) // Hide the OS-specific "chrome" around the window
      .with_inner_size([400.0, 100.0])
      .with_min_inner_size([400.0, 100.0])
      .with_transparent(true), // To have rounded corners we need transparency
    ..Default::default()
  };
  eframe::run_native("overlay", options, Box::new(|_cc| Box::<MyApp>::default()))
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array()
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
      .frame(egui::Frame {
        ..Default::default()
      })
      .show(ctx, |_ui| {});
    egui::Window::new("hey").show(ctx, |_ui| {});
  }
}
