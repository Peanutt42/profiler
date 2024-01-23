use eframe::egui;
use crate::ProcessedProfiler;
use profiler::Profiler;
use std::path::Path;

pub struct Viewer {
	show_open_file_dialog: bool,
	loading_error_msg: Option<String>,
	offset: f32,
	zoom: f32,
	screen_width: f32,
	mouse_pos: egui::Pos2,
	profiler: Option<ProcessedProfiler>,
}

impl Viewer {
	pub fn new() -> Self {
		Viewer {
            show_open_file_dialog: true,
            loading_error_msg: None,
            offset: 0.0,
            zoom: 1.0,
            screen_width: 800.0,
            mouse_pos: egui::Pos2::new(0.0, 0.0),
            profiler: None,
        }
	}

	pub fn update(&mut self, ctx: &egui::Context) {
		self.handle_drag_and_drop(ctx);

		self.handle_input(ctx);

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.profiler.is_none() {
				return;
			}

			if let Some(profiler) = &self.profiler {
				let canvas = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("profile_results")));

				self.screen_width = ctx.screen_rect().width();
				let center_x = self.screen_width / 2.0;
				let height = 20.0;

				for frame in profiler.frames.iter() {
					let frame_start_pixel = center_x + (frame.start.as_secs_f32() * self.screen_width / profiler.total_time.as_secs_f32() - self.offset) * self.zoom;
					let frame_end_pixel = center_x + ((frame.start + frame.duration).as_secs_f32() * self.screen_width / profiler.total_time.as_secs_f32() - self.offset) * self.zoom;
					if frame_start_pixel > self.screen_width || frame_end_pixel < 0.0 {
						continue;
					}
					
					for profile_result in frame.profile_results.iter() {
						let x = center_x + (profile_result.start.as_secs_f32() * self.screen_width / profiler.total_time.as_secs_f32() - self.offset) * self.zoom;
						let y = profile_result.depth as f32 * height;
						let width = (profile_result.duration.as_secs_f32() / profiler.total_time.as_secs_f32()) * self.screen_width * self.zoom;
						
						let rect = egui::Rect::from_min_size(egui::Pos2::new(x, y), egui::Vec2::new(width, height));
						canvas.rect(rect, 2.5, egui::Color32::BLUE, egui::Stroke::new(1.5, egui::Color32::BLACK));

						let mut allow_tooltip = false;
						if width > 50.0 {
							let truncated = Self::draw_truncated_text(&canvas, ui, &profile_result.name, width, rect.center());
							if truncated {
                                allow_tooltip = true;
                            }
						}
						else {
							allow_tooltip = true;
						}
						if allow_tooltip && self.mouse_pos.x >= x && self.mouse_pos.y >= y && self.mouse_pos.y <= y + height && self.mouse_pos.x <= x + width {
							egui::show_tooltip_at_pointer(ctx, egui::Id::new("profiler_result_tooltip"), |ui| {
								ui.label(&profile_result.name);
							});
						}
					}
				}
			}
		});

		if self.show_open_file_dialog {
			self.open_file_dialog(ctx);
		}
	}

	fn handle_input(&mut self, ctx: &egui::Context) {
		ctx.input(|i| {
			if let Some(pos) = i.pointer.latest_pos() {
				self.mouse_pos = pos;
			}

			for e in i.events.iter() {
				match e {
					egui::Event::MouseWheel { unit: _, delta, modifiers: _ } => {
						let factor = delta.y * 0.15 + 1.0;
						self.zoom *= factor;
						self.offset -= (self.mouse_pos.x - (self.screen_width / 2.0)) / self.zoom * ((1.0 / factor) - 1.0);
					}
					_ => {},
				}
			}

			if i.pointer.primary_down() {
				self.offset -= i.pointer.delta().x / self.zoom;
			}
			if i.pointer.secondary_down() {
				// just zooms at the center
				self.zoom *= i.pointer.delta().y * 0.005 + 1.0;
			}
		});
	}

	fn handle_drag_and_drop(&mut self, ctx: &egui::Context) {
		ctx.input(|i| {
			for file in i.raw.dropped_files.iter() {
				let mut loaded_profiler = Profiler::new();
				if let Err(e) = loaded_profiler.load_from_file(Path::new(&file.path.clone().unwrap())) {
					self.loading_error_msg = Some(e.to_string());
					self.show_open_file_dialog = true;
				}
				else {
					self.loading_error_msg = None;
					self.profiler = Some(ProcessedProfiler::new(&loaded_profiler));
					self.show_open_file_dialog = false;
				}
			}
		});
	}

	fn open_file_dialog(&mut self, ctx: &egui::Context) {
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
							self.loading_error_msg = Some(e);
						}
						else {
							self.loading_error_msg = None;
							self.profiler = Some(ProcessedProfiler::new(&loaded_profiler));
							self.show_open_file_dialog = false;
						}
					}
				}

				if let Some(error) = self.loading_error_msg.clone() {
					ui.visuals_mut().override_text_color = Some(egui::Color32::RED);

					ui.label(&error);
				}
			});
	}


	fn glyph_width(text: String, font_id: egui::FontId, canvas: &egui::Painter) -> f32 {
		canvas.layout_no_wrap(text, font_id, egui::Color32::WHITE).rect.width()
	}
	fn glyph_char_width(c: char, font_id: egui::FontId, ui: &mut egui::Ui) -> f32 {
		ui.fonts(|f| f.glyph_width(&font_id, c))
	}

	// returns wheter the text was truncated
	fn draw_truncated_text(painter: &egui::Painter, ui: &mut egui::Ui, text: &str, max_width: f32, pos: egui::Pos2) -> bool {
		let font_id = egui::TextStyle::Body.resolve(ui.style());

		let text_width = Self::glyph_width(text.to_string(), font_id.clone(), painter);
		
		let truncated_text = if text_width > max_width {
			let ellipsis_width = Self::glyph_width("...".to_string(), font_id.clone(), painter);
			let mut current_width = 0.0;
			let mut truncated_length = 0;
			for (i, char_width) in text.chars().map(|c| Self::glyph_char_width(c, font_id.clone(), ui)).enumerate() {
				current_width += char_width;
				if current_width + ellipsis_width > max_width {
					break;
				}
				truncated_length = i + 1;
			}

			let mut truncated_text = String::with_capacity(truncated_length + 3);
			truncated_text.push_str(&text[..truncated_length]);
			truncated_text.push_str("...");
			truncated_text
		} else {
			text.to_owned()
		};

		painter.text(pos, egui::Align2::CENTER_CENTER, truncated_text, font_id, egui::Color32::WHITE);

		text_width > max_width
	}
}