use std::time::{Instant, Duration};
use std::cell::RefCell;

mod chrome_tracing;
mod function_name;

#[derive(Clone)]
pub struct Scope {
	pub name: String,
    pub start: Instant,
	pub duration: Duration,
}

impl Scope {
	pub fn new(name: String) -> Self {
		//let mut start = 0;
		//PROFILER.with(|p| start = Instant::now().duration_since(p.borrow().program_start).as_millis());
		//println!("{} started {}ms", name, start);

		Self {
			name,
			start: Instant::now(),
			duration: Duration::new(0, 0),
		}
	}
}

impl Drop for Scope {
	fn drop(&mut self) {
		self.duration = self.start.elapsed();
        let _ = PROFILER.try_with(|p| {
			if let Ok(mut p) = p.try_borrow_mut() {
				p.submit_scope(self.name.clone(), self.start, self.duration);
			}
		});
    }
}

#[macro_export]
macro_rules! scope {
	($name:expr) => {
		let _scope = profiler::Scope::new(format!("{}::{}", profiler::function_name!(), $name));
	};
}


struct Frame {
	scopes: Vec<Scope>,
}

impl Frame {
	fn new() -> Self {
		Self {
			scopes: Vec::new(),
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

	fn submit_scope(&mut self, name: String, start: Instant, duration: Duration) {
		if self.frames.is_empty() {
			self.frames.push(Frame::new());
		}

		self.frames.last_mut().unwrap().scopes.push(Scope { name, start, duration });
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