use profiler::{new_frame, scope, save_to_file};
use profiler_attributes::profile;
use std::path::Path;

#[profile]
fn work() {
	println!("work");
}

fn main() {
	for i in 0..2000 {
		new_frame!();

		scope!(format!("frame_{i}"));

		work();
	}

	save_to_file!(&Path::new("saved.profiling"));
}