use profiler::{submit_frame, scope, save_to_file};
use profiler_attributes::profile;

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
		{
			scope!(format!("frame_{i}"));

			work_1();
			work_2();
		}
		submit_frame!();
	}

	save_to_file!("saved.profiling");
}