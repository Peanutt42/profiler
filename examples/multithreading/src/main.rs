use profiler::{save_to_file, scope, submit_frame};

fn worker_thread() {
	for _ in 0..10 {
		{
			scope!("background work");
			std::thread::sleep(std::time::Duration::from_millis(100));
		}
		submit_frame!();
	}
}

fn main() {
	let mut worker_threads = Vec::new();
	for i in 0..10 {
		worker_threads.push(std::thread::Builder::new().name(format!("worker thread {i}")).spawn(worker_thread).unwrap());
	}

	for thread in worker_threads {
		thread.join().unwrap();
	}

	save_to_file!("saved.profiling");
}