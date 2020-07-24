use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {

    let path = env::var("OUT_DIR")
        .and_then(|dir| Ok(Path::new(&dir).join("version.rs")))
        .expect("Failed to read environment variable OUT_DIR");

    let content = concat!(
        "pub fn get_version() -> &'static str { \"",
            env!("CARGO_PKG_VERSION"),
         "\"}");

    File::create(&path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .expect("Failed to write version.rs file");
}