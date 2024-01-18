use std::time::{Instant, Duration};
use std::cell::RefCell;

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
        PROFILER.with(|p| p.borrow_mut().submit_dynamic_scope(&self.name, self.start, self.start.elapsed()));
    }
}

#[macro_export]
macro_rules! scope {
	($name:expr) => {
		profiler::Scope::new(format!("{}::{}", profiler::function_name!(), $name))
	};
}

pub struct CustomScope<'a> {
	pub name: String,
    pub start: Instant,
	profiler: &'a mut Profiler,
}

impl<'a> CustomScope<'a> {
	pub fn new(name: String, profiler: &'a mut Profiler) -> Self {
		Self {
			name,
            start: Instant::now(),
            profiler,
		}
	}
}

impl<'a> Drop for CustomScope<'a> {
	fn drop(&mut self) {
		let duration = self.start.elapsed();
		self.profiler.submit_dynamic_scope(&self.name, self.start, duration);
	}
}

#[macro_export]
macro_rules! custom_scope {
	($profiler:expr) => {
		profiler::CustomScope::new(profiler::function_name!().to_string(), $profiler)
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

	pub fn submit_scope(&mut self, name: &str, start: Instant, duration: Duration) {
		println!("{} took {}ms", name, duration.as_millis());
    }

	pub fn submit_dynamic_scope(&mut self, name: &String, start: Instant, duration: Duration) {
		println!("{} took {}ms", name, duration.as_millis());
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