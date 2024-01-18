use profiler::{Profiler, Scope, PROFILER, new_frame, profile_scope, function_name};

#[test]
fn hello_world() {
	for i in 0..10 {
		let scope = Scope::new(function_name!());

		PROFILER.with(|p| p.borrow_mut().new_frame());
	}
}

#[test]
fn hello_world_macros() {
	for i in 0..10 {
		profile_scope!();

		new_frame!();
	}
}