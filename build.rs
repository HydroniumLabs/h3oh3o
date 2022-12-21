use cbindgen::Config;
use std::{env, path::PathBuf};

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    let output_file = target_dir().join("h3api.h").display().to_string();

    let config =
        Config::from_file(PathBuf::from(crate_dir).join("cbindgen.toml"))
            .expect("configure cbindgen");

    cbindgen::generate_with_config(crate_dir, config)
        .expect("generate binding")
        .write_to_file(&output_file);
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CH3O_EXPORT_DIR") {
        // Build through CMake.
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
