use std::time::Duration;
use profiler::Profiler;

#[derive(Clone)]
pub struct ProcessedProfileResult {
	pub name: String,
	pub start: Duration,
    pub duration: Duration,
    pub self_duration: Duration,
	pub depth: usize,
}

impl ProcessedProfileResult {
	fn is_inside(&self, other: &Self) -> bool {
		let self_end = self.start + self.duration;
		let other_end = other.start + other.duration;
		self.start >= other.start && self_end <= other_end
	}
}

#[derive(Clone)]
pub struct ProcessedFrame {
	pub start: Duration,
	pub duration: Duration,
	pub profile_results: Vec<ProcessedProfileResult>,
}

#[derive(Clone)]
pub struct ProcessedProfiler {
	pub total_time: Duration,
	pub frames: Vec<ProcessedFrame>,
}

impl ProcessedProfiler {
	pub fn new(profiler: Profiler) -> Self {
		let mut frames = Vec::with_capacity(profiler.frames.len());

		let mut total_time = Duration::from_secs(0);
		for profile_result in profiler.frames.last().unwrap().profile_results.iter() {
			let end_time = profile_result.start + profile_result.duration;
			if total_time < end_time {
				total_time = end_time;
			}
		}

		for frame in profiler.frames {
			let mut profile_results: Vec<ProcessedProfileResult> = Vec::new();
			for profile_result in frame.profile_results {
				profile_results.push(ProcessedProfileResult {
					name: profile_result.name,
					start: profile_result.start,
					duration: profile_result.duration,
					self_duration: profile_result.duration,
					depth: profile_result.depth,
				});
			}

			for i in 0..profile_results.len() {
				for j in 0..profile_results.len() {
					if i == j {
						continue;
					}

					if profile_results[i].depth + 1 == profile_results[j].depth && profile_results[j].is_inside(&profile_results[i]) {
						let non_self_duration = profile_results[j].duration;
						profile_results[i].self_duration -= non_self_duration;
					}
				}
			}

			frames.push(
				ProcessedFrame {
                    start: frame.start,
                    duration: frame.duration,
					profile_results,
                }
			);
		}

		Self { total_time, frames }
	}
}