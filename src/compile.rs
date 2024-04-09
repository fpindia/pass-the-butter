use std::process::Command;

// Puts the dylib in the same dir as the source
pub fn compile(file: &str, output_dir: &str) {
    // let current_dir = std::env::current_dir().unwrap();
    Command::new("rustc")
        .arg(file)
        .arg("--crate-type")
        .arg("dylib")
        .arg("--out-dir")
        .arg(output_dir)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}
