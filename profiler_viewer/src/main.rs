use eframe::egui;
use profiler_gui::Viewer;

fn main() -> eframe::Result<()>{
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
		..Default::default()
	};
	
	let mut viewer = Viewer::new();

	eframe::run_simple_native("Profiler GUI", options, move |ctx, _frame| {
		egui::CentralPanel::default().show(ctx, |ui| {
			viewer.update(ui);
		});
	})
}