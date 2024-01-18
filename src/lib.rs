use std::time::{Instant, Duration};
use std::cell::RefCell;

pub struct Scope {
	pub name: &'static str,
    pub start: Instant,
	pub duration: Duration,
}

impl Scope {
	pub fn new(name: &'static str) -> Self {
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
		println!("{} took {}", self.name, self.duration.as_secs_f32());
	}
}

#[macro_export]
macro_rules! profile_scope {
    () => {
        let scope = profiler::Scope::new(profiler::function_name!());
    };
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }}
}

pub struct Profiler {

}

impl Profiler {
	pub fn new() -> Self {
		Self {

		}
	}

	pub fn new_frame(&mut self) {

	}
}

thread_local! {
	pub static PROFILER: RefCell<Profiler> = RefCell::new(Profiler::new());
}

#[macro_export]
macro_rules! new_frame {
	() => {
		{
			PROFILER.with(|p| p.borrow_mut().new_frame());
		}
	};
}