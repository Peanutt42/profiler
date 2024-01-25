use eframe::egui;

mod processed_profiler;
use processed_profiler::ProcessedProfiler;

mod viewer;
use viewer::Viewer;

fn main() -> eframe::Result<()>{
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
		..Default::default()
	};
	
	let mut viewer = Viewer::new();

	eframe::run_simple_native("Profiler GUI", options, move |ctx, _frame| {
		viewer.update(ctx);
	})
}