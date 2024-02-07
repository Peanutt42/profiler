use std::time::Duration;
use profiler::{Profiler, Frame};

#[derive(Clone)]
pub struct ProcessedProfiler {
	pub total_time: Duration,
	pub frames: Vec<Frame>,
}

impl ProcessedProfiler {
	pub fn new(profiler: Profiler) -> Self {
		let mut total_time = Duration::from_secs(0);
		for profile_result in profiler.frames.last().unwrap().profile_results.iter() {
			let end_time = profile_result.start + profile_result.duration;
			if total_time < end_time {
				total_time = end_time;
			}
		}

		Self { total_time, frames: profiler.frames }
	}
}