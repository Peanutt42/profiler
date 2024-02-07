use profiler::{new_frame, scope, save_to_file};
use profiler_attributes::profile;
use std::path::Path;

#[profile]
fn work_1(depth: usize) {
	if depth == 0 {
		return;
	}

	println!("work_1");

	work_2(depth - 1);
}

#[profile]
fn work_2(depth: usize) {
	if depth == 0 {
		return;
	}

	println!("work_2");

	work_1(depth - 1);
}

fn main() {
	for i in 0..10000 {
		new_frame!();

		scope!(format!("frame_{i}"));

		// just so frame_x and work_1 don't overlap, just ignore, not really needed
		//std::thread::sleep(std::time::Duration::from_nanos(1));

		work_1(100);
		work_2(100);
	}

	save_to_file!(&Path::new("saved.yaml"));
}