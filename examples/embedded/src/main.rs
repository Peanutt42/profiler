use eframe::egui;
use profiler::submit_frame;
use profiler_attributes::profile;
use std::sync::{Mutex, Arc};

#[profile]
fn work() {
	std::thread::sleep(std::time::Duration::from_millis(1));
}

fn background_thread(how_much_work: Arc<Mutex<i32>>, quit: Arc<Mutex<bool>>) {
	while !*quit.lock().unwrap() {
		let how_much_work = *how_much_work.lock().unwrap();
		for _ in 0..how_much_work {
			work();
		}
		
		submit_frame!();
	}
}

fn main() -> eframe::Result<()> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
		..Default::default()
	};
	let mut viewer = profiler_gui::Viewer::new();
	
	let how_much_work = Arc::new(Mutex::new(5));
	let how_much_work_clone = how_much_work.clone();
	let how_much_work_clone2 = how_much_work.clone();
	
	let quit_background_thread = Arc::new(Mutex::new(false));
	let quit_background_thread_clone = quit_background_thread.clone();
	let background_thread = std::thread::Builder::new()
		.name("background_thread".to_string())
		.spawn(|| background_thread(how_much_work_clone2, quit_background_thread_clone))
		.unwrap();
	
	let result = eframe::run_simple_native("Embedded Example", options, move |ctx, _frame| {
		ctx.request_repaint();

		egui::CentralPanel::default().show(ctx, |ui| {
			let mut new_how_much_work = *how_much_work_clone.lock().unwrap();
			ui.add(egui::Slider::new(&mut new_how_much_work, 1..=100).text("How much work"));
			*how_much_work_clone.lock().unwrap() = new_how_much_work;
		});

		egui::Window::new("Embedded Profiler").show(ctx, |ui| {
			viewer.update_embedded();
			viewer.update(ui);
		});
	});

	*quit_background_thread.lock().unwrap() = true;
	background_thread.join().unwrap();
	result
}