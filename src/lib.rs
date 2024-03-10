use std::{hash::{Hash, Hasher}, time::{Duration, Instant}};
#[cfg(not(feature = "disable_profiling"))]
use std::cell::RefCell;
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

mod function_name;
mod serialization;
mod scope;
#[cfg(not(feature = "disable_profiling"))]
pub use scope::Scope;
pub use scope::ScopeResult;


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
	#[cfg(not(feature = "disable_profiling"))]
	pub static PROFILER: RefCell<Profiler> = RefCell::new(Profiler::new());
}

pub struct Profiler {
	current_frame: Frame,
	#[cfg(not(feature = "disable_profiling"))]
	current_frame_call_depth: usize,
	program_start: Instant,
}

impl Profiler {
	pub fn new() -> Self {
		let program_start = Instant::now();
		Self {
			current_frame: Frame::new(&program_start),
			#[cfg(not(feature = "disable_profiling"))]
			current_frame_call_depth: 0,
			program_start,
		}
	}

	pub fn submit_frame(&mut self) {
		self.current_frame.duration = std::time::Instant::now().duration_since(self.program_start) - self.current_frame.start;
		let thread_id = get_current_thread_id_u64();
		let mut global_profiler = GLOBAL_PROFILER.lock().unwrap();
		global_profiler.thread_profilers
			.entry(thread_id)
			.or_default()
			.frames.push(self.current_frame.clone());
		self.current_frame = Frame::new(&self.program_start);
	}

	#[cfg(not(feature = "disable_profiling"))]
	fn begin_profile_result(&mut self) {
		self.current_frame_call_depth += 1;
	}

	#[cfg(not(feature = "disable_profiling"))]
	fn submit_profile_result(&mut self, name: String, start: Instant, duration: Duration) {
		self.current_frame.scope_results.push(ScopeResult::new(name, start.duration_since(self.program_start), duration, self.current_frame_call_depth - 1));
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
macro_rules! submit_frame {
	() => {
		{
			profiler::PROFILER.with_borrow_mut(|p| p.submit_frame());
		}
	};
}

#[macro_export]
#[cfg(feature = "disable_profiling")]
macro_rules! submit_frame {
	() => {
		
	};
}


fn get_current_thread_id_u64() -> u64 {
	let thread_id = std::thread::current().id();
	let mut hasher = std::hash::DefaultHasher::default();
	thread_id.hash(&mut hasher);
	hasher.finish()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreadProfiler {
	pub name: String,
	pub frames: Vec<Frame>,
}

impl ThreadProfiler {
	pub fn new(name: String, frames: Vec<Frame>) -> Self {
		Self {
			name,
			frames,
		}
	}
}

impl Default for ThreadProfiler {
	fn default() -> Self {
		let thread_name = if let Some(thread_name) = std::thread::current().name() {
			thread_name.to_string()
		}
		else {
			"Unnamed Thread".to_string()
		};
		Self::new(thread_name, Vec::new())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalProfiler {
	pub thread_profilers: HashMap<u64, ThreadProfiler>,
}

impl GlobalProfiler {
	pub fn new() -> Self {
		Self {
			thread_profilers: HashMap::new(),
		}
	}
}

impl Default for GlobalProfiler {
	fn default() -> Self {
		Self::new()
	}
}

pub static GLOBAL_PROFILER: Lazy<Mutex<GlobalProfiler>> = Lazy::new(|| Mutex::new(GlobalProfiler::new()));