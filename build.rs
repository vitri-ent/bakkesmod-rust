use std::{env, path::PathBuf};

#[cfg(docsrs)]
fn main() {}

#[cfg(not(docsrs))]
fn main() {
	let crate_root: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
	let sdk_path = crate_root.join("lib");

	println!("cargo:rustc-link-search={}", sdk_path.display());
	println!("cargo:rustc-link-lib=BakkesPluginSDK180");
}
