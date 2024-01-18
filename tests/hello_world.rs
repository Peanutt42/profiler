use profiler::{Profiler, new_frame, scope, custom_scope};
use profiler_attributes::profile;

#[profile]
fn work() {
	std::thread::sleep(std::time::Duration::from_millis(10));
}

#[test]
fn simple() {
	for i in 0..10 {
		work();

		new_frame!();
	}
	for i in 0..10 {
		{
			let _scope = scope!("scope_task");
            std::thread::sleep(std::time::Duration::from_millis(10));
		}

		new_frame!();
	}
}


#[test]
fn custom() {
	let mut profiler = Profiler::new();

	for i in 0..10 {
		{
			let _scope = custom_scope!(&mut profiler);
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
		profiler.new_frame();
	}
}