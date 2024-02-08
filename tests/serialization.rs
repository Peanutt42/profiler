#[cfg(not(feature = "disable_profiling"))]
use profiler::{Profiler, PROFILER, new_frame};
#[cfg(not(feature = "disable_profiling"))]
use profiler_attributes::profile;

#[cfg(not(feature = "disable_profiling"))]
#[profile]
fn work() {
	std::thread::sleep(std::time::Duration::from_millis(50));
}

#[test]
#[cfg(not(feature = "disable_profiling"))]
fn serialization_test() {
	{
		for _ in 0..10 {
			new_frame!();

			work();
		}
	}

	let bytes = PROFILER.with_borrow_mut(|p| p.to_binary());
	let mut new_profiler = Profiler::new();
	new_profiler.from_binary(&bytes.expect("failed to generate binary from profiler")).expect("failed to parse binary for profiler");
	assert_eq!(new_profiler.frames.len(), 10);
	assert_eq!(new_profiler.frames[0].scope_results.len(), 1);
	assert_eq!(new_profiler.frames[0].scope_results[0].name, "serialization::work".to_string());
	assert!(new_profiler.frames[0].scope_results[0].duration >= std::time::Duration::from_millis(50));
}