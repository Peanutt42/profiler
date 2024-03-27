
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f).strip_suffix("::f").unwrap();
		let mut cleaned_name = String::new();
		let mut prev_end = 0;
		
		for (start, _) in name.match_indices("::{{closure}}") {
			cleaned_name.push_str(&name[prev_end..start]);
			prev_end = start + "::{{closure}}".len();
		}
		
		cleaned_name.push_str(&name[prev_end..]);
		
		cleaned_name
    }}
}
