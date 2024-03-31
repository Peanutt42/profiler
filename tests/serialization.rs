#[cfg(feature = "enable_profiling")]
use profiler::{GlobalProfiler, GLOBAL_PROFILER, submit_frame};
#[cfg(feature = "enable_profiling")]
use profiler_attributes::profile;

#[cfg(feature = "enable_profiling")]
#[profile]
fn work() {
	std::thread::sleep(std::time::Duration::from_millis(50));
}

#[test]
#[cfg(feature = "enable_profiling")]
fn serialization_test() {
	{
		for _ in 0..10 {
			work();

			submit_frame!();
		}
	}

	let bytes = GLOBAL_PROFILER.lock().unwrap().to_binary();
	let mut new_profiler = GlobalProfiler::new();
	new_profiler.from_binary(&bytes.expect("failed to generate binary from profiler")).expect("failed to parse binary for profiler");
	assert_eq!(new_profiler.thread_profilers.len(), 1);
	for thread_profiler in new_profiler.thread_profilers.values() {
		assert_eq!(thread_profiler.frames.len(), 10);
		assert_eq!(thread_profiler.frames[0].scope_results.len(), 1);
		assert_eq!(thread_profiler.frames[0].scope_results[0].name, "serialization::work".to_string());
		assert!(thread_profiler.frames[0].scope_results[0].duration >= std::time::Duration::from_millis(50));
	}
}