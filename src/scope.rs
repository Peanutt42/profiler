

#[cfg(feature = "enable_profiling")]
use std::time::Instant;
use std::time::Duration;
use serde::{Serialize, Deserialize};
#[cfg(feature = "enable_profiling")]
use crate::PROFILER;

#[derive(Clone)]
#[cfg(feature = "enable_profiling")]
pub struct Scope {
	pub name: String,
    pub start: Instant,
}

#[cfg(feature = "enable_profiling")]
impl Scope {
	pub fn new(name: String) -> Self {
		PROFILER.with_borrow_mut(|p| p.begin_profile_result());
		Self {
			name,
			start: Instant::now(),
		}
	}
}

#[cfg(feature = "enable_profiling")]
impl Drop for Scope {
	fn drop(&mut self) {
		let duration = self.start.elapsed();

        PROFILER.with_borrow_mut(|p| p.submit_profile_result(self.name.clone(), self.start, duration));
    }
}

#[macro_export]
#[cfg(feature = "enable_profiling")]
macro_rules! scope {
	($name:expr) => {
		let _scope = profiler::Scope::new(format!("{}::{}", profiler::function_name!(), $name));
	};
}

#[macro_export]
#[cfg(not(feature = "enable_profiling"))]
macro_rules! scope {
	($name:expr) => {
		
	};
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ScopeResult {
	pub name: String,
    pub start: Duration,
    pub duration: Duration,
	pub depth: usize,
}

impl ScopeResult {
	pub fn new(name: String, start: Duration, duration: Duration, depth: usize) -> Self {
		Self {
            name,
            start,
            duration,
			depth,
        }
	}
	
	pub fn is_inside(&self, other: &Self) -> bool {
		let self_end = self.start + self.duration;
		let other_end = other.start + other.duration;
		self.start >= other.start && self_end <= other_end
	}
}