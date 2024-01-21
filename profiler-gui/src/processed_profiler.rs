use std::time::Duration;
use profiler::Profiler;

#[derive(Clone)]
pub struct ProcessedProfileResult {
	pub name: String,
	pub start: Duration,
    pub duration: Duration,
	pub depth: usize,
}

#[derive(Clone)]
pub struct ProcessedFrame {
	pub start: Duration,
	pub duration: Duration,
	pub profile_results: Vec<ProcessedProfileResult>,
}

#[derive(Clone)]
pub struct ProcessedProfiler {
	pub frames: Vec<ProcessedFrame>,
}

impl ProcessedProfiler {
	pub fn new(profiler: &Profiler) -> Self {
		let mut frames = Vec::with_capacity(profiler.frames.len());

		for frame in profiler.frames.iter() {
			let mut frame_start_time = Duration::from_secs(0);
			let mut frame_duration = Duration::from_secs(0);
			let mut profile_results: Vec<ProcessedProfileResult> = Vec::new();
			for profile_result in frame.profile_results.iter() {
				if frame_start_time < profile_result.start {
					frame_start_time = profile_result.start;
				}
				if frame_duration < profile_result.duration {
					frame_duration = profile_result.duration;
                }

				let mut depth = 0;
				for other_profile_result in frame.profile_results.iter() {
					if profile_result.start >= other_profile_result.start && (profile_result.start + profile_result.duration) <= (other_profile_result.start + other_profile_result.duration) {
						depth += 1;
					}
				}

				profile_results.push(ProcessedProfileResult { name: profile_result.name.clone(), start: profile_result.start, duration: profile_result.duration, depth });
			}

			frames.push(
				ProcessedFrame {
                    start: frame_start_time,
                    duration: frame_duration,
					profile_results,
                }
			);
		}

		Self { frames, }
	}
}