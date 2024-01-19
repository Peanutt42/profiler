use proc_macro::TokenStream;
use syn::{parse_macro_input,ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn profile(_args: TokenStream, input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ItemFn);
	let function_name = &input.sig.ident;
	let body = &input.block;

	let expanded = quote! {
		fn #function_name() {
			let __profile_start = std::time::Instant::now();
			let __profile_function_name = profiler::function_name!();
			let result = {
				#body
			};
			profiler::PROFILER.with(|p| p.borrow_mut().submit_scope(__profile_function_name, __profile_start, __profile_start.elapsed()));
			result
		}
	};
	TokenStream::from(expanded)
}