use crate::{Profiler,Frame};
use std::path::Path;
use std::fs::File;
use std::io::Write;

impl Profiler {
	pub fn from_yaml(&mut self, yaml: &String) {
		let deserialized: serde_yaml::Result<Vec<Frame>> = serde_yaml::from_str(&yaml);
		match deserialized {
			Ok(frames) => {
				self.frames = frames;
			},
			Err(e) => {
				println!("Failed to parse YAML{}", e);
			}
		}
	}

	pub fn load_from_file(&mut self, filepath: &Path) {
		match std::fs::read_to_string(filepath) {
			Ok(yaml) => {
				self.from_yaml(&yaml);
			},
			Err(e) => {
				println!("Failed to read from file {}: {}", filepath.display(), e);
			}
		}
	}

	pub fn to_yaml(&self) -> String {
		match serde_yaml::to_string(&self.frames) {
			Ok(yaml) => {
				yaml
			},
			Err(e) => {
				panic!("Failed to serialize YAML: {}", e);
			}
		}
	}

	pub fn save_to_file(&self, path: &Path) {
		match File::create(path) {
			Ok(mut file) => {
				let _ = file.write_all(self.to_yaml().as_bytes());
			},
			Err(e) => {
				println!("Failed to write to file {}: {}", path.display(), e);
			}
		}
	}
}

#[macro_export]
macro_rules! save_to_file {
	($filepath:expr) => {
		profiler::PROFILER.with(|p| p.borrow_mut().save_to_file($filepath));
	};
}