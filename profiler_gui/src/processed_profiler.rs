use std::{collections::HashMap, time::Duration};
use profiler::ThreadProfiler;

#[derive(Debug, Clone)]
pub struct ProcessedGlobalProfiler {
	pub total_time: Duration,
	pub thread_profilers: HashMap<u64, ThreadProfiler>,
}

impl ProcessedGlobalProfiler {
	pub fn new(thread_profilers: HashMap<u64, ThreadProfiler>) -> Self {
		let mut total_time = Duration::from_secs(0);
		for thread_profiler in thread_profilers.values() {
			for scope_result in thread_profiler.frames.last().unwrap().scope_results.iter() {
				let end_time = scope_result.start + scope_result.duration;
				if total_time < end_time {
					total_time = end_time;
				}
			}
		}

		Self {
			total_time,
			thread_profilers,
		}
	}
}