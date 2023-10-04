/// This build script is used to extract the app name from Cargo.toml and pass it to the compiler
/// as an environment variable. This is used by the eframe crate to set the window title.
/// Txs gpt-4, the only change was to use `cargo fmt` and `cargo clippy`:
///   https://chat.openai.com/share/3db475d5-8f22-4f8d-8e43-eda07ed54a51
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
    let cargo_toml_content =
        fs::read_to_string(cargo_toml_path).expect("Unable to read Cargo.toml");

    let cargo_toml: toml::Value =
        toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");
    let app_name = cargo_toml
        .get("package")
        .and_then(|pkg| pkg.get("name"))
        .and_then(|name| name.as_str());

    if let Some(app_name) = app_name {
        println!("cargo:rustc-env=APP_NAME={}", app_name);
    }
}
