# profiler
simple rust instrumentation profiler
```rust
#[profile]
fn work_1() {
	std::thread::sleep(std::time::Duration::from_millis(500));
}

#[profile]
fn work_2() {
	std::thread::sleep(std::time::Duration::from_millis(500));
}

fn main() {
	for i in 0..4 {
		new_frame!();

		scope!(format!("frame_{i}"));

		work_1();
		work_2();
	}

	save_to_file!(&Path::new("saved.profiling"));
}
```
![alt text](screenshot.png)
