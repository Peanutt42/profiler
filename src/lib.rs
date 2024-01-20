use std::time::{Instant, Duration};
use std::cell::RefCell;

mod chrome_tracing;
mod function_name;

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
			p.borrow_mut().submit_profile_result(ProfileResult::new(self.name.clone(), self.start, duration));
		});
    }
}

#[macro_export]
macro_rules! scope {
	($name:expr) => {
		let _scope = profiler::Scope::new(format!("{}::{}", profiler::function_name!(), $name));
	};
}

struct ProfileResult {
	name: String,
    start: Instant,
    duration: Duration,
}

impl ProfileResult {
	pub fn new(name: String, start: Instant, duration: Duration) -> Self {
		Self {
            name,
            start,
            duration,
        }
	}
}


struct Frame {
	profile_results: Vec<ProfileResult>,
}

impl Frame {
	fn new() -> Self {
		Self {
			profile_results: Vec::new(),
		}
	}
}

pub struct Profiler {
	frames: Vec<Frame>,
	program_start: Instant,
}

impl Profiler {
	fn new() -> Self {
		Self {
			frames: Vec::new(),
			program_start: Instant::now(),
		}
	}

	pub fn new_frame(&mut self) {
		self.frames.push(Frame::new());
	}

	fn submit_profile_result(&mut self, profile_result: ProfileResult) {
		if self.frames.is_empty() {
			self.frames.push(Frame::new());
		}

		self.frames.last_mut().unwrap().profile_results.push(profile_result);
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