use profiler::{new_frame, scope, save_to_chrome_tracing};
use profiler_attributes::profile;
use std::path::Path;

#[profile]
fn work_1() {
	println!("work_1");

	std::thread::sleep(std::time::Duration::from_millis(500));
}

#[profile]
fn work_2() {
	println!("work_2");

	std::thread::sleep(std::time::Duration::from_millis(500));
}


fn main() {
	for i in 0..4 {
		new_frame!();

		{
			scope!(format!("frame_{i}"));

			// just so frame_x and work_1 don't overlap, just ignore, not really needed
			std::thread::sleep(std::time::Duration::from_nanos(10));

			work_1();
			work_2();
		}
	}

	save_to_chrome_tracing!(&Path::new("chrome_tracing.json"));
}