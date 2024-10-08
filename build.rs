use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the directory where the build script is being run
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(out_dir).join("../../../");

    // Define the source and destination paths
    let src = PathBuf::from("private_key.pem");
    let dest = target_dir.join("private_key.pem");

    // Copy the file
    fs::copy(&src, &dest).expect("Failed to copy private_key.pem to target directory");
}
