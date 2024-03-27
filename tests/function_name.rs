use profiler::function_name;

#[test]
fn normal_function_name() {
	assert_eq!(function_name!(), "function_name::normal_function_name");
}

mod a_mod {
	use profiler::function_name;

	#[test]
	fn function_in_mod() {
		assert_eq!(function_name!(), "function_name::a_mod::function_in_mod");

		// should remove closures
		let _closure = || {
			assert_eq!(function_name!(), "function_name::a_mod::function_in_mod");
		};
	}
}