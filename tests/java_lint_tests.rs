
use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_working_java_files() {
    let linter_cmd = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug") // Use debug build for tests
        .join("lintymclintface");

    let working_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("working");

    for entry in fs::read_dir(&working_dir).expect("Failed to read tests/working directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "java") {
            println!("Testing working file: {:?}", path);
            let output = Command::new(&linter_cmd)
                .arg("-l")
                .arg("java")
                .arg("-f")
                .arg(&path)
                .output()
                .expect("Failed to execute linter command");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stderr.is_empty() {
                eprintln!("Stderr for {:?}: {}", path, stderr);
            }

            assert!(
                stdout.trim().is_empty() || stdout.trim() == "[]",
                "Expected no errors for {:?}, but got: {}",
                path,
                stdout
            );
        }
    }
}

#[test]
fn test_failing_java_files() {
    let linter_cmd = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug") // Use debug build for tests
        .join("lintymclintface");

    let failing_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("failing");

    for entry in fs::read_dir(&failing_dir).expect("Failed to read tests/failing directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "java") {
            println!("Testing failing file: {:?}", path);
            let output = Command::new(&linter_cmd)
                .arg("-l")
                .arg("java")
                .arg("-f")
                .arg(&path)
                .output()
                .expect("Failed to execute linter command");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stderr.is_empty() {
                eprintln!("Stderr for {:?}: {}", path, stderr);
            }

            // For failing files, we expect some JSON output (errors)
            assert!(
                !stdout.trim().is_empty() && stdout.trim() != "[]",
                "Expected errors for {:?}, but got no output or empty array: {}",
                path,
                stdout
            );
            // TODO: In a more robust solution, we would parse the JSON and assert specific error details.
        }
    }
}
