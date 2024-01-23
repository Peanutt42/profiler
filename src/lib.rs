use std::time::{Instant, Duration};
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

mod chrome_tracing;
mod function_name;
mod serialization;

#[derive(Clone)]
pub struct Scope {
	pub name: String,
    pub start: Instant,
}

impl Scope {
	pub fn new(name: String) -> Self {
		Self {
			name,
			start: Instant::now(),
		}
	}
}

impl Drop for Scope {
	fn drop(&mut self) {
		let duration = self.start.elapsed();

        PROFILER.with(|p| {
			p.borrow_mut().submit_profile_result(self.name.clone(), self.start, duration);
		});
    }
}

#[macro_export]
macro_rules! scope {
	($name:expr) => {
		let _scope = profiler::Scope::new(format!("{}::{}", profiler::function_name!(), $name));
	};
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfileResult {
	pub name: String,
    pub start: Duration,
    pub duration: Duration,
}

impl ProfileResult {
	pub fn new(name: String, start: Duration, duration: Duration) -> Self {
		Self {
            name,
            start,
            duration,
        }
	}
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
	pub start: Duration,
	pub duration: Duration,
	pub profile_results: Vec<ProfileResult>,
}

impl Frame {
	fn new(program_start: &Instant) -> Self {
		Self {
			start: Instant::now().duration_since(*program_start),
			duration: Duration::from_secs(0),
			profile_results: Vec::new(),
		}
	}
}

pub struct Profiler {
	pub frames: Vec<Frame>,
	program_start: Instant,
}

impl Profiler {
	pub fn new() -> Self {
		Self {
			frames: Vec::new(),
			program_start: Instant::now(),
		}
	}

	pub fn new_frame(&mut self) {
		self.finish_last_frame();
		self.frames.push(Frame::new(&self.program_start));
	}

	pub fn finish_last_frame(&mut self) {
		if let Some(last_frame) = self.frames.last_mut() {
			last_frame.duration = std::time::Instant::now().duration_since(self.program_start) - last_frame.start;
		}
	}

	fn submit_profile_result(&mut self, name: String, start: Instant, duration: Duration) {
		if self.frames.is_empty() {
			self.frames.push(Frame::new(&self.program_start));
		}

		self.frames.last_mut().unwrap().profile_results.push(ProfileResult::new(name, start.duration_since(self.program_start), duration));
	}
}

thread_local! {
	pub static PROFILER: RefCell<Profiler> = RefCell::new(Profiler::new());
}

#[macro_export]
macro_rules! new_frame {
	() => {
		{
			profiler::PROFILER.with(|p| p.borrow_mut().new_frame());
		}
	};
}