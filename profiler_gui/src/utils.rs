use eframe::egui;

pub fn glyph_width(text: String, font_id: egui::FontId, canvas: &egui::Painter) -> f32 {
	canvas.layout_no_wrap(text, font_id, egui::Color32::WHITE).rect.width()
}
pub fn glyph_char_width(c: char, font_id: egui::FontId, ui: &mut egui::Ui) -> f32 {
	ui.fonts(|f| f.glyph_width(&font_id, c))
}

// returns wheter the text was truncated
pub fn draw_truncated_text(painter: &egui::Painter, ui: &mut egui::Ui, text: &str, max_width: f32, pos: egui::Pos2) -> bool {
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