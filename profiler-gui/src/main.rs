use eframe::egui;
use std::path::Path;
use profiler::Profiler;

mod processed_profiler;
use processed_profiler::ProcessedProfiler;

fn main() -> eframe::Result<()>{
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
		..Default::default()
	};

	let mut show_open_file_dialog = true;
	let mut loading_error_msg: Option<String> = None;

	let mut profiler: Option<ProcessedProfiler> = None;

	eframe::run_simple_native("Profiler GUI", options, move |ctx, _frame| {
		ctx.input(|i| {
			for file in i.raw.dropped_files.iter() {
				let mut loaded_profiler = Profiler::new();
				if let Err(e) = loaded_profiler.load_from_file(Path::new(&file.path.clone().unwrap())) {
					loading_error_msg = Some(e.to_string());
					show_open_file_dialog = true;
				}
				else {
					loading_error_msg = None;
					profiler = Some(ProcessedProfiler::new(&loaded_profiler));
					show_open_file_dialog = false;
				}
			}
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			if profiler.is_none() {
				return;
			}

			if let Some(profiler) = &profiler {
				let canvas = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("profile_results")));
				
				let mut total_time = 0.0;
				if !profiler.frames.is_empty() {
					for profile_result in profiler.frames.last().unwrap().profile_results.iter() {
						let end_time = (profile_result.start + profile_result.duration).as_secs_f32();
						if total_time < end_time {
							total_time = end_time;
						}
					}
				}

				let screen_width = ctx.screen_rect().width();

				for frame in profiler.frames.iter() {
					for profile_result in frame.profile_results.iter() {
						let x = profile_result.start.as_secs_f32() * screen_width / total_time;
						let width = (profile_result.duration.as_secs_f32() / total_time) * screen_width;
						
						let rect_height = 20.0;
						let rect = egui::Rect::from_min_size(egui::Pos2::new(x, profile_result.depth as f32 * rect_height), egui::Vec2::new(width, rect_height));
						canvas.rect(rect, 2.5, egui::Color32::BLUE, egui::Stroke::new(1.5, egui::Color32::BLACK));

						let font_color = egui::Color32::WHITE;
						let text = canvas.layout_no_wrap(profile_result.name.clone(), egui::TextStyle::Body.resolve(ui.style()), font_color);
						if text.rect.width() <= width {
							canvas.galley(rect.center() - text.rect.size() / 2.0, text, font_color);
						}
					}
				}
			}
		});

		if show_open_file_dialog {
			egui::Window::new("Open saved profiling record")
				.default_size([300.0, 150.0])
				.collapsible(false)
				.movable(false)
				.anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
				.show(ctx, |ui|
			{
				ui.label("Drag and drop a file or ...");

				if ui.button("Load").clicked() {
					if let Some(filepath) =  rfd::FileDialog::new().add_filter("YAML", &["yaml", "yml"]).pick_file() {
						let mut loaded_profiler = Profiler::new();
						if let Err(e) = loaded_profiler.load_from_file(&Path::new(&filepath)) {
							loading_error_msg = Some(e);
						}
						else {
							loading_error_msg = None;
							profiler = Some(ProcessedProfiler::new(&loaded_profiler));
							show_open_file_dialog = false;
						}
					}
				}

				if let Some(error) = loading_error_msg.clone() {
					ui.visuals_mut().override_text_color = Some(egui::Color32::RED);

					ui.label(&error);
				}
			});
		}
	})
}