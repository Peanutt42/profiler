use crate::Profiler;
use std::path::Path;
use std::fs::File;
use std::io::Write;

impl Profiler {
	pub fn from_yaml(&mut self, yaml: &str) -> serde_yaml::Result<()> {
		self.frames = serde_yaml::from_str(yaml)?;

		Ok(())
	}

	pub fn load_from_file(&mut self, filepath: &Path) -> Result<(), String> {
		match std::fs::read_to_string(filepath) {
			Ok(yaml) => {
				if let Err(e) = self.from_yaml(&yaml) {
					return Err(e.to_string());
				}
				Ok(())
			},
			Err(e) => {
				Err(e.to_string())
			}
		}
	}

	pub fn to_yaml(&mut self) -> serde_yaml::Result<String> {
		self.finish_last_frame();
		serde_yaml::to_string(&self.frames)
	}

	pub fn save_to_file(&mut self, path: &Path) -> Result<(), String> {
		let file = File::create(path);
		if let Err(e) = file {
			return Err(e.to_string());
		}

		let yaml = self.to_yaml();
		if let Err(e) = yaml {
			return Err(e.to_string());
		}

		if let Err(e) = file.unwrap().write_all(yaml.unwrap().as_bytes()) {
			return Err(e.to_string());
		}

		Ok(())
	}
}

#[macro_export]
macro_rules! save_to_file {
	($filepath:expr) => {
		if let Err(e) = profiler::PROFILER.with(|p| p.borrow_mut().save_to_file($filepath)) {
			println!("Failed to write to file {}: {}", $filepath.display(), e);
		}
	};
}