use eframe::egui;
use std::path::Path;
use profiler::Profiler;

fn main() -> eframe::Result<()>{
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
		..Default::default()
	};

	let mut show_open_file_dialog = true;
	let mut loading_error_msg: Option<String> = None;

	let mut profiler = Profiler::new();

	eframe::run_simple_native("Profiler GUI", options, move |ctx, _frame| {
		egui::CentralPanel::default().show(ctx, |ui| {
			if profiler.frames.len() == 0 {
				return;
			}

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

			let rect_height = 20.0;
			let screen_width = ctx.screen_rect().width();
					
			for frame in profiler.frames.iter() {
				for profile_result in frame.profile_results.iter() {
					let x = (profile_result.start.as_secs_f32() / total_time) * screen_width;
					let width = (profile_result.duration.as_secs_f32() / total_time) * screen_width;

					// calculate height
					let mut y_index = 0;
					for other_profile_result in frame.profile_results.iter() {
						if profile_result.start >= other_profile_result.start && (profile_result.start + profile_result.duration) <= (other_profile_result.start + other_profile_result.duration) {
							y_index += 1;
						}
					}
					let y = y_index as f32 * rect_height;

					let rect = egui::Rect::from_min_size(egui::Pos2::new(x, y), egui::Vec2::new(width, rect_height));
					canvas.rect_filled(rect, 2.5, if y_index % 2 == 0 { egui::Color32::RED } else { egui::Color32::BLUE });

					

					// Display function name as tooltip
					if width > 30.0 {
						canvas.text(
							rect.center(),
							egui::Align2::CENTER_CENTER,
							&profile_result.name,
							egui::TextStyle::Body.resolve(ui.style()),
							egui::Color32::WHITE,
						);
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
				if ui.button("Load").clicked() {
					if let Some(filepath) =  rfd::FileDialog::new().add_filter("YAML", &["yaml", "yml"]).pick_file() {
						if let Err(e) = profiler.load_from_file(&Path::new(&filepath)) {
							loading_error_msg = Some(e);
						}
						else {
							loading_error_msg = None;
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