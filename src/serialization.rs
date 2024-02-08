use crate::Profiler;
use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::Write;

impl Profiler {
	pub fn from_binary(&mut self, bytes: &[u8]) -> bincode::Result<()> {
		self.frames = bincode::deserialize(bytes)?;

		Ok(())
	}

	pub fn load_from_file(&mut self, filepath: &Path) -> Result<()> {
		let bytes = std::fs::read(filepath)?;
		self.from_binary(&bytes)?;
		Ok(())
	}

	pub fn to_binary(&mut self) -> bincode::Result<Vec<u8>> {
		self.finish_last_frame();
		bincode::serialize(&self.frames)
	}

	pub fn save_to_file(&mut self, path: &Path) -> Result<()> {
		let mut file = File::create(path)?;
		let bytes = self.to_binary()?;
		file.write_all(&bytes)?;
		Ok(())
	}
}

#[macro_export]
#[cfg(not(feature = "disable_profiling"))]
macro_rules! save_to_file {
	($filepath:expr) => {
		if let Err(e) = profiler::PROFILER.with_borrow_mut(|p| p.save_to_file($filepath)) {
			println!("Failed to write to file {}: {}", $filepath.display(), e);
		}
	};
}

#[macro_export]
#[cfg(feature = "disable_profiling")]
macro_rules! save_to_file {
	($filepath:expr) => {
		
	};
}