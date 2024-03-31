use crate::GlobalProfiler;
use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::Write;

impl GlobalProfiler {
	pub fn from_binary(&mut self, bytes: &[u8]) -> bincode::Result<()> {
		self.thread_profilers = bincode::deserialize(bytes)?;

		Ok(())
	}

	pub fn load_from_file(&mut self, filepath: &Path) -> Result<()> {
		let bytes = std::fs::read(filepath)?;
		self.from_binary(&bytes)?;
		Ok(())
	}

	pub fn to_binary(&mut self) -> bincode::Result<Vec<u8>> {
		bincode::serialize(&self.thread_profilers)
	}

	pub fn save_to_file<P>(&mut self, path: P) -> Result<()>
	where P: AsRef<Path>
	{
		let mut file = File::create(path)?;
		let bytes = self.to_binary()?;
		file.write_all(&bytes)?;
		Ok(())
	}
}

#[macro_export]
#[cfg(feature = "enable_profiling")]
macro_rules! save_to_file {
	($filepath:expr) => {
		profiler::GLOBAL_PROFILER.lock().unwrap().save_to_file($filepath).expect(concat!("Failed to write to file {}", $filepath));
	};
}

#[macro_export]
#[cfg(not(feature = "enable_profiling"))]
macro_rules! save_to_file {
	($filepath:expr) => {
		
	};
}