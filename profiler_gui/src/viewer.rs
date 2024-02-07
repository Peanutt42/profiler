use eframe::egui;
use crate::ProcessedProfiler;
use profiler::Profiler;
use std::{path::Path, time::Duration};

const TIMELINE_HEIGHT: f64 = 15.0;

pub struct Viewer {
	show_open_file_dialog: bool,
	loading_error_msg: Option<String>,
	view_start: f64,
	view_end: f64,
	smooth_view_start: f64,
	smooth_view_end: f64,
	offset_y: f64,
	screen_width: f64,
	screen_height: f64,
	mouse_pos: egui::Pos2,
	profiler: Option<ProcessedProfiler>,
}

impl Viewer {
	pub fn new() -> Self {
		Viewer {
            show_open_file_dialog: true,
            loading_error_msg: None,
            view_start: 0.0,
            view_end: 1.0,
            smooth_view_start: 0.0,
            smooth_view_end: 1.0,
            offset_y: 0.0,
            screen_width: 800.0,
            screen_height: 600.0,
            mouse_pos: egui::Pos2::new(0.0, 0.0),
            profiler: None,
        }
	}

	fn calc_pos_x(&self, x: f64) -> f64 {
		if let Some(profiler) = &self.profiler {
			let relative_pos = x / profiler.total_time.as_secs_f64();
			(self.smooth_view_start * (1.0 - relative_pos) + self.smooth_view_end * relative_pos) * self.screen_width
		}
		else {
			0.0
		}
	}

	pub fn update(&mut self, ctx: &egui::Context) {
		self.handle_drag_and_drop(ctx);

		self.handle_input(ctx);

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.profiler.is_none() {
				return;
			}
			let profiler = self.profiler.as_ref().unwrap();

			let canvas = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("profile_results")));

			let padding = 10.0;
			
			self.screen_width = ctx.screen_rect().width() as f64;
			self.screen_height = ctx.screen_rect().height() as f64;
			let rounding = 2.5;
			let function_height = 28.0;
			let hover_rect_offset = 1.0;
			
			let mut selection_rect: Option<egui::Rect> = None;
			
			for frame in profiler.frames.iter() {
				let frame_start_pixel = self.calc_pos_x(frame.start.as_secs_f64());
				let frame_end_pixel = self.calc_pos_x((frame.start + frame.duration).as_secs_f64());
				if frame_start_pixel > self.screen_width || frame_end_pixel < 0.0 {
					continue;
				}
				
				for (i, profile_result) in frame.profile_results.iter().enumerate() {
					let x = self.calc_pos_x(profile_result.start.as_secs_f64());
					let y = profile_result.depth as f64 * function_height + TIMELINE_HEIGHT + padding - self.offset_y;
					let width = self.calc_pos_x((profile_result.start + profile_result.duration).as_secs_f64()) - x;
					
					if x > self.screen_width || x + width < 0.0 || y > self.screen_height {
						continue;
					}
					
					let rect = egui::Rect::from_min_size(egui::Pos2::new(x as f32, y as f32), egui::Vec2::new(width as f32, function_height as f32));
					canvas.rect(rect, rounding, egui::Color32::BLUE, egui::Stroke::new(1.5, egui::Color32::BLACK));
					
					let mut show_name_tooltip = false;
					if width > 50.0 {
						let truncated = draw_truncated_text(&canvas, ui, &profile_result.name, width as f32, rect.center());
						if truncated {
							show_name_tooltip = true;
						}
					}
					else {
						show_name_tooltip = true;
					}
					
					let hovered: bool = self.mouse_pos.x as f64 >= x && self.mouse_pos.y as f64 >= y && self.mouse_pos.y as f64 <= y + function_height && self.mouse_pos.x as f64 <= x + width;
					if hovered {
						selection_rect = Some(egui::Rect::from_min_size(rect.min - egui::Vec2::new(hover_rect_offset, hover_rect_offset), rect.size() + egui::Vec2::new(2.0 * hover_rect_offset, 2.0 * hover_rect_offset)));
						
						egui::show_tooltip_at_pointer(ctx, egui::Id::new("profiler_result_tooltip"), |ui| {
							if show_name_tooltip {
								ui.label(&profile_result.name);
							}
							ui.label(format!("Duration: {}", format_duration(&profile_result.duration)));

							let mut self_duration = profile_result.duration;
							for j in 0..frame.profile_results.len() {
								if i == j {
									continue;
								}
			
								if profile_result.depth + 1 == frame.profile_results[j].depth && frame.profile_results[j].is_inside(profile_result) {
									self_duration -= frame.profile_results[j].duration;
								}
							}
							ui.label(format!("Self Duration: {}", format_duration(&self_duration)));
						});
					}
				}
			}
			if let Some(selection_rect) = selection_rect {
				canvas.rect_stroke(selection_rect, rounding, egui::Stroke::new(2.0 * hover_rect_offset, egui::Color32::YELLOW));
			}

			self.draw_timeline(&canvas, ctx);
		});		
		
		if self.show_open_file_dialog {
			self.open_file_dialog(ctx);
		}
	}

	fn draw_timeline(&self, canvas: &egui::Painter, ctx: &egui::Context) {
		if self.profiler.is_none() || self.profiler.as_ref().unwrap().frames.is_empty() {
			return;
		}

		let profiler = self.profiler.as_ref().unwrap();
		
		let screen_width = ctx.screen_rect().width() as f64;

		// start      0             width    end
		//  |         #################        |
		let start = self.calc_pos_x(0.0);
		let end = self.calc_pos_x(profiler.total_time.as_secs_f64());

		let left_percentage = (0.0 - start) / (end - start);
		let right_percentage = (screen_width - start) / (end - start);
		let view_start = left_percentage * screen_width;
		let view_end = right_percentage * screen_width;

		canvas.rect_filled(egui::Rect::from_min_max(egui::Pos2::new(view_start as f32, 0.0), egui::Pos2::new(view_end as f32, TIMELINE_HEIGHT as f32)), 0.0, egui::Color32::WHITE);

		for frame in profiler.frames.iter() {
			canvas.rect(egui::Rect::from_min_size(egui::Pos2::new((frame.start.as_secs_f64() / profiler.total_time.as_secs_f64()) as f32 * screen_width as f32, 0.0), egui::Vec2::new(1.0, TIMELINE_HEIGHT as f32)), 0.0, egui::Color32::GRAY, egui::Stroke::new(1.0, egui::Color32::BLACK));
		}
	}

	fn handle_input(&mut self, ctx: &egui::Context) {
		ctx.input(|i| {
			if let Some(pos) = i.pointer.latest_pos() {
				self.mouse_pos = pos;
			}

			// 0.0..1.0 inside the current view
			let mouse_pos_relative_to_view = ((self.mouse_pos.x as f64 / self.screen_width) - self.view_start) / (self.view_end - self.view_start);
			let zoom_target = self.view_start + (self.view_end - self.view_start) * mouse_pos_relative_to_view;
					
			for e in i.events.iter() {
				if let egui::Event::MouseWheel { unit: _, delta, modifiers: _ } = e {
					self.zoom(-0.1 * delta.y as f64, zoom_target);
				}
			}

			if i.pointer.primary_down() {
				let mouse_delta = i.pointer.delta();
				self.view_start += mouse_delta.x as f64 / self.screen_width;
				self.view_end += mouse_delta.x as f64 / self.screen_width;
				self.offset_y -= mouse_delta.y as f64;
			}
			if i.pointer.secondary_down() {
				self.zoom(-0.005 * i.pointer.delta().y as f64, zoom_target);
			}
		});

		let smooth_start_difference = self.view_start - self.smooth_view_start;
		let smooth_end_difference = self.view_end - self.smooth_view_end;
		if smooth_start_difference.abs() > 0.01 || smooth_end_difference.abs() > 0.01 {
			ctx.request_repaint();
		}
		let stable_dt = ctx.input(|i| i.stable_dt as f64);
		self.smooth_view_start += smooth_start_difference * 7.5 * stable_dt;
		self.smooth_view_end += smooth_end_difference * 7.5 * stable_dt;
		self.offset_y = self.offset_y.max(0.0);
	}

	fn zoom(&mut self, amount: f64, zoom_target: f64) {
		self.view_start += (zoom_target - self.view_start) * amount;
		self.view_end -= (self.view_end - zoom_target) * amount;
	}

	fn load_profiler(&mut self, filepath: &Path) {
		let mut loaded_profiler = Profiler::new();
		if let Err(e) = loaded_profiler.load_from_file(filepath) {
			self.loading_error_msg = Some(e.to_string());
			self.show_open_file_dialog = true;
		}
		else {
			self.loading_error_msg = None;
			self.profiler = Some(ProcessedProfiler::new(loaded_profiler));
			self.view_start  = 0.0;
			self.view_end = 1.0;
			self.offset_y = 0.0;
			self.show_open_file_dialog = false;
		}
	}

	fn handle_drag_and_drop(&mut self, ctx: &egui::Context) {
		ctx.input(|i| {
			for file in i.raw.dropped_files.iter() {
				self.load_profiler(file.path.as_ref().unwrap());
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
				if let Some(filepath) =  rfd::FileDialog::new().add_filter("Profiling", &["profiling"]).pick_file() {
					self.load_profiler(&filepath);
				}
			}

			if let Some(error) = self.loading_error_msg.clone() {
				ui.visuals_mut().override_text_color = Some(egui::Color32::RED);

				ui.label(&error);
			}
		});
	}
}

fn format_duration(duration: &Duration) -> String {
	const NANOS_PER_SEC: f32 = 1_000_000_000.0;
	const NANOS_PER_MILLI: f32 = 1_000_000.0;
	const NANOS_PER_MICRO: f32 = 1_000.0;
	const MILLIS_PER_SEC: f32 = 1_000.0;
	const MICROS_PER_SEC: f32 = 1_000_000.0;

	let secs = duration.as_secs() as f32;
	let nanos = duration.subsec_nanos() as f32;
	let secs_f32 = secs + nanos / NANOS_PER_SEC;
	if secs_f32 >= 0.1 {
		return format!("{secs_f32} s");
	}
	let millis = secs * MILLIS_PER_SEC + nanos / NANOS_PER_MILLI;
	if millis >= 0.1 {
		return format!("{millis} ms");
	}
	let micros = secs * MICROS_PER_SEC + nanos / NANOS_PER_MICRO;
	if micros >= 0.1 {
		return format!("{micros} us");
	}
	let nanos = secs * NANOS_PER_SEC + nanos;
	format!("{nanos} ns")
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

	let text_width = glyph_width(text.to_string(), font_id.clone(), painter);
	
	if text_width > max_width {
		let mut current_width = 0.0;
		let mut truncated_length = 0;
		for (i, char_width) in text.chars().map(|c| glyph_char_width(c, font_id.clone(), ui)).enumerate() {
			current_width += char_width;
			if current_width > max_width {
				break;
			}
			truncated_length = i + 1;
		}

		painter.text(pos, egui::Align2::CENTER_CENTER, &text[..truncated_length], font_id, egui::Color32::WHITE);
		true
	} else {
		painter.text(pos, egui::Align2::CENTER_CENTER, text, font_id, egui::Color32::WHITE);
		false
	}
}