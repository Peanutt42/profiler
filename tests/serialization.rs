use profiler::{Profiler, PROFILER, new_frame};
use profiler_attributes::profile;

#[profile]
fn work() {
	std::thread::sleep(std::time::Duration::from_millis(50));
}

#[test]
fn serialization_test() {
	{
		for _ in 0..10 {
			new_frame!();

			work();
		}
	}

	let yaml = PROFILER.with(|p| p.borrow_mut().to_yaml());
	let mut new_profiler = Profiler::new();
	new_profiler.from_yaml(&yaml.expect("failed to generate yaml from profiler")).expect("failed to parse yaml for profiler");
	assert_eq!(new_profiler.frames.len(), 10);
	assert_eq!(new_profiler.frames[0].profile_results.len(), 1);
	assert_eq!(new_profiler.frames[0].profile_results[0].name, "serialization::work".to_string());
	assert!(new_profiler.frames[0].profile_results[0].duration >= std::time::Duration::from_millis(50));
}