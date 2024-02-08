use std::time::{Instant, Duration};
#[cfg(not(feature = "disable_profiling"))]
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

mod function_name;
mod serialization;
mod scope;
pub use scope::{Scope, ScopeResult};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frame {
	pub start: Duration,
	pub duration: Duration,
	pub scope_results: Vec<ScopeResult>,
}

impl Frame {
	fn new(program_start: &Instant) -> Self {
		Self {
			start: Instant::now().duration_since(*program_start),
			duration: Duration::from_secs(0),
			scope_results: Vec::new(),
		}
	}
}


thread_local! {
	#[macro_export]
	#[cfg(not(feature = "disable_profiling"))]
	pub static PROFILER: RefCell<Profiler> = RefCell::new(Profiler::new());
}

pub struct Profiler {
	pub frames: Vec<Frame>,
	current_frame_call_depth: usize,
	program_start: Instant,
}

impl Profiler {
	pub fn new() -> Self {
		Self {
			frames: Vec::new(),
			current_frame_call_depth: 0,
			program_start: Instant::now(),
		}
	}

	pub fn new_frame(&mut self) {
		self.finish_last_frame();
		self.frames.push(Frame::new(&self.program_start));
		assert!(self.current_frame_call_depth == 0);
	}

	pub fn finish_last_frame(&mut self) {
		if let Some(last_frame) = self.frames.last_mut() {
			last_frame.duration = std::time::Instant::now().duration_since(self.program_start) - last_frame.start;
		}
	}

	#[cfg(not(feature = "disable_profiling"))]
	fn begin_profile_result(&mut self) {
		self.current_frame_call_depth += 1;
	}

	#[cfg(not(feature = "disable_profiling"))]
	fn submit_profile_result(&mut self, name: String, start: Instant, duration: Duration) {
		if self.frames.is_empty() {
			self.frames.push(Frame::new(&self.program_start));
		}

		self.frames.last_mut().unwrap().scope_results.push(ScopeResult::new(name, start.duration_since(self.program_start), duration, self.current_frame_call_depth - 1));
		self.current_frame_call_depth -= 1;
	}
}

impl Default for Profiler {
	fn default() -> Self {
		Self::new()
	}
}


#[macro_export]
#[cfg(not(feature = "disable_profiling"))]
macro_rules! new_frame {
	() => {
		{
			profiler::PROFILER.with_borrow_mut(|p| p.new_frame());
		}
	};
}

#[macro_export]
#[cfg(feature = "disable_profiling")]
macro_rules! new_frame {
	() => {
		
	};
}