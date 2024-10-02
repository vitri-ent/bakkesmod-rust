extern crate proc_macro;

use std::env;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{LitByteStr, LitStr};

#[proc_macro_attribute]
pub fn plugin_init(args: TokenStream, input: TokenStream) -> TokenStream {
	let mut plugin_name = (env::var("CARGO_PKG_NAME").unwrap(), Span::call_site());
	let mut plugin_version = (env::var("CARGO_PKG_VERSION").unwrap(), Span::call_site());
	{
		let mut first = true;
		let args_parser = syn::meta::parser(|meta| {
			let was_first = first;
			first = false;

			if was_first && meta.path.get_ident().is_some() {
				let ident = meta.path.get_ident().unwrap();
				plugin_name = (ident.to_string(), ident.span());
				Ok(())
			} else {
				Err(meta.error("unsupported property"))
			}
		});
		syn::parse_macro_input!(args with args_parser);
	}

	let plugin_name = LitByteStr::new(&[plugin_name.0.as_bytes(), &[0]].concat(), plugin_name.1);
	let plugin_version = LitByteStr::new(&[plugin_version.0.as_bytes(), &[0]].concat(), plugin_version.1);

	let parsed_input: syn::ItemFn = syn::parse_macro_input!(input as syn::ItemFn);
	let name = parsed_input.clone().sig.ident;

	let tokens = quote! {
		#parsed_input

		#[used]
		#[export_name = "exports"]
		static __BM_EXPORTS: ::bakkesmod::internal::PluginInfo = ::bakkesmod::internal::PluginInfo {
			api_build_version: 95,
			file_name: #plugin_name.as_ptr(),
			class_name: #plugin_name.as_ptr(),
			plugin_name: #plugin_name.as_ptr(),
			plugin_version: #plugin_version.as_ptr(),
			plugin_type: 0,
			initialize_func: ::bakkesmod::internal::bmrs_pluginInit,
			del_func: ::bakkesmod::internal::bmrs_pluginUninit
		};

		#[no_mangle]
		pub extern "C" fn bmrs_pluginOnLoad(cvm: *mut (), game: *mut ()) {
			::bakkesmod::internal::bakkesmod_init(cvm, game);
			#name();
		}

		#[no_mangle]
		pub extern "C" fn bmrs_pluginOnUnload() {
			::bakkesmod::internal::bakkesmod_exit();
		}
	};

	tokens.into()
}
