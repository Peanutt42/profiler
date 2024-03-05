use profiler::{submit_frame, scope, save_to_file};
use profiler_attributes::profile;

#[profile]
fn work() {
	println!("work");
}

fn main() {
	for i in 0..2000 {
		{
			scope!(format!("frame_{i}"));

			work();
		}
		submit_frame!();
	}

	save_to_file!("saved.profiling");
}