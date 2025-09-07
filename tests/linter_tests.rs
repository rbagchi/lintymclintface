use std::process::Command;
use std::fs;
use std::path::PathBuf;
use serde_json::from_str;
use lintymclintface::SyntaxError;

// Helper function to run linter and get output
fn run_linter_test(file_path: &PathBuf, language: &str) -> (String, String) {
    let linter_cmd = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug") // Use debug build for tests
        .join("lintymclintface");

    let output = Command::new(&linter_cmd)
        .arg("-l")
        .arg(language)
        .arg("-f")
        .arg(&file_path)
        .output()
        .expect("Failed to execute linter command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stderr.is_empty() {
        eprintln!("Stderr for {:?}: {}", file_path, stderr);
    }
    (stdout, stderr)
}

#[test]
fn test_working_java_files() {
    let working_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("working");

    for entry in fs::read_dir(&working_dir).expect("Failed to read tests/working directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "java") {
            println!("Testing working Java file: {:?}", path);
            let (stdout, _) = run_linter_test(&path, "java");
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
    let failing_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("failing");

    for entry in fs::read_dir(&failing_dir).expect("Failed to read tests/failing directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "java") {
            println!("Testing failing Java file: {:?}", path);
            let (stdout, _) = run_linter_test(&path, "java");
            assert!(
                !stdout.trim().is_empty() && stdout.trim() != "[]",
                "Expected errors for {:?}, but got no output or empty array: {}",
                path,
                stdout
            );
            // TODO: Add specific assertions for Java failing files if needed
        }
    }
}

#[test]
fn test_working_python_files() {
    let working_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("working");

    for entry in fs::read_dir(&working_dir).expect("Failed to read tests/working directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "py") {
            println!("Testing working Python file: {:?}", path);
            let (stdout, _) = run_linter_test(&path, "python");
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
fn test_failing_python_files() {
    let failing_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("failing");

    for entry in fs::read_dir(&failing_dir).expect("Failed to read tests/failing directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "py") {
            println!("Testing failing Python file: {:?}", path);
            let (stdout, _) = run_linter_test(&path, "python");

            let errors: Vec<SyntaxError> = from_str(&stdout)
                .expect(&format!("Failed to parse JSON output for {:?}: {}", path, stdout));

            if path.file_name().unwrap() == "syntax_error_python.py" {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].line, 1);
                assert_eq!(errors[0].column, 1);
                assert!(errors[0].message.contains("Syntax error near 'def my_func\n    pass'"));
            } else {
                // General assertion for other failing Python files
                assert!(
                    !stdout.trim().is_empty() && stdout.trim() != "[]",
                    "Expected errors for {:?}, but got no output or empty array: {}",
                    path,
                    stdout
                );
            }
        }
    }
}

#[test]
fn test_working_r_files() {
    let working_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("working");

    for entry in fs::read_dir(&working_dir).expect("Failed to read tests/working directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "R") {
            println!("Testing working R file: {:?}", path);
            let (stdout, _) = run_linter_test(&path, "r");
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
fn test_failing_r_files() {
    let failing_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("failing");

    for entry in fs::read_dir(&failing_dir).expect("Failed to read tests/failing directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "R") {
            println!("Testing failing R file: {:?}", path);
            let (stdout, _) = run_linter_test(&path, "r");

            let errors: Vec<SyntaxError> = from_str(&stdout)
                .expect(&format!("Failed to parse JSON output for {:?}: {}", path, stdout));

            if path.file_name().unwrap() == "syntax_error_r.R" {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].line, 1);
                assert_eq!(errors[0].column, 19);
                assert!(errors[0].message.contains("Missing )"));
            } else {
                // General assertion for other failing R files
                assert!(
                    !stdout.trim().is_empty() && stdout.trim() != "[]",
                    "Expected errors for {:?}, but got no output or empty array: {}",
                    path,
                    stdout
                );
            }
        }
    }
}