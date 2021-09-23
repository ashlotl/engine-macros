use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn glue_runloop(ast: TokenStream) -> TokenStream {
	let loop_name: Ident = parse_macro_input!(ast);
	let fn_name: Ident = format_ident!("new_runloop_{}", loop_name);
	(quote! {
		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn #fn_name(
			//TODO: put in macro pls
			from: &String,
			id: RunLoopId,
		) -> Result<Box<dyn RunLoop>, ron::Error> {
			let created: Result<#loop_name, ron::Error> = ron::from_str(from.as_str());

			match created {
				Ok(mut v) => {
					v.run_loop_id = id;
					return Ok(Box::new(v));
				}
				Err(e) => return Err(e),
			}
		}
	})
	.into()
}
