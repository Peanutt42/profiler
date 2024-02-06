use crate::Profiler;
use std::io::Write;
use std::path::Path;
use std::fs::File;

impl Profiler {
	pub fn save_to_chrome_tracing(&self, filepath: &Path) {
		let file = File::create(filepath);
		match file {
			Ok(mut file) => {
				if let Err(e) = self.write_chrome_tracing_json(&mut file) {
					println!("Failed to write to file {}: {}", filepath.display(), e);
				}
			},
			Err(e) => {
                println!("Failed to save chrome tracing to {}: {}", filepath.display(), e);
            }
		}
	}

	fn write_chrome_tracing_profile_results(&self, file: &mut File) -> std::io::Result<()> {
		for frame in self.frames.iter() {
			for profile_result in frame.profile_results.iter() {
				file.write_all(b",{")?;
				file.write_all(b"\"cat\":\"function\",")?;
				file.write_all(format!("\"dur\":{},", profile_result.duration.as_micros()).as_bytes())?;
				file.write_all(format!("\"name\":\"{}\",", profile_result.name).as_bytes())?;
				file.write_all(b"\"ph\":\"X\",")?;
				file.write_all(b"\"pid\":0,")?;
				file.write_all(b"\"tid\":0,")?;
				file.write_all(format!("\"ts\":{}", profile_result.start.as_micros()).as_bytes())?;
				file.write_all(b"}")?;
			}
		}

		Ok(())
	}

	fn write_chrome_tracing_json(&self, file: &mut File) -> std::io::Result<()>{
		file.write_all(b"{\"otherData\":{},\"traceEvents\":[{}")?;

		self.write_chrome_tracing_profile_results(file)?;

		file.write_all(b"]}")?;

		file.flush()?;

		Ok(())
	}
}


#[macro_export]
macro_rules! save_to_chrome_tracing {
	($filepath:expr) => {
		{
			profiler::PROFILER.with(|p| p.borrow_mut().save_to_chrome_tracing($filepath));
		}
	};
}