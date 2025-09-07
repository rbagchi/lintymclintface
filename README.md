# lintymclintface: A Fast, Modular Syntax Checker for Python, Java, and R

<table>
  <tr>
    <td><img src="mascot.png" alt="Lintymclintface Mascot" width="200"></td>
    <td>`lintymclintface` is a high-performance, modular syntax checker designed to quickly identify common errors in Python, Java, and R code. Built with Rust and leveraging the power of Tree-sitter, it provides both a command-line interface (CLI) for local use and a RESTful web service for integration into automated systems.</td>
  </tr>
</table>

## Features

*   **Fast Syntax Checking**: Utilizes Tree-sitter for efficient and accurate parsing.
*   **Multi-language Support**: Supports Python, Java, and R.
*   **Command-Line Interface (CLI)**: Easily lint individual files from your terminal.
*   **Web Service (REST API)**: Integrate linting into your CI/CD pipelines or other automated workflows via HTTP POST requests.
*   **Structured Error Output**: Provides syntax errors in a machine-readable JSON format, ideal for programmatic consumption.
*   **Prometheus Metrics**: When running as a service, exposes Prometheus-compatible metrics for monitoring linting requests and performance.

## Getting Started

### Prerequisites

Before you can build and run `lintymclintface`, you'll need the following installed on your system:

*   **Rust Toolchain**: The Rust programming language and its package manager, Cargo. You can install `rustup` (the Rust toolchain installer) by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
*   **Git**: For cloning the repository and the example repositories. Download from [git-scm.com](https://git-scm.com/downloads).
*   **cURL** (optional, for testing the web service): A command-line tool for making HTTP requests. Usually pre-installed on macOS and Linux. For Windows, you can download it from the [official cURL website](https://curl.se/download.html).

### Code Quality

To ensure code quality and consistency, `lintymclintface` uses `rustfmt` for code formatting and `clippy` for linting. It's recommended to run these tools before committing changes.

*   **Format Code:**
    ```bash
    cargo fmt --all
    ```
    To check for formatting issues without applying them:
    ```bash
    cargo fmt --all -- --check
    ```

*   **Run Linter (Clippy):**
    ```bash
    cargo clippy --all-targets -- -D warnings
    ```
    This command runs Clippy on all targets and treats linter warnings as errors, ensuring a high standard of code quality.

### Building from Source

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/rbagchi/lintymclintface.git
    cd lintymclintface
    ```

2.  **Build the project in release mode**:
    ```bash
    cargo build --release
    ```
    This command compiles the project and creates an optimized executable binary located at `target/release/lintymclintface`.

## Usage

### Command-Line Interface (CLI)

To lint a single file using the CLI, specify the language and the file path:

```bash
target/release/lintymclintface -l <language> -f <file_path>
```

*   `<language>`: Can be `python`, `java`, or `r`.
*   `<file_path>`: The absolute or relative path to the file you want to lint.

**Example: Linting a Python file**

First, let's create a dummy Python file with a syntax error (e.g., `test_python_error.py`):

```python
# test_python_error.py
def my_function():
    print("Hello, world!"
```

Now, run the linter:

```bash
target/release/lintymclintface -l python -f test_python_error.py
```

Expected JSON output:

```json
[
  {
    "line": 2,
    "column": 25,
    "message": "Syntax error near '\n    print(\"Hello, world!\"'"
  }
]
```

*(Note: The exact line and column numbers might vary slightly depending on the specific error and file content.)*

**Example: Linting a Java file**

Create a dummy Java file with a syntax error (e.g., `Test.java`):

```java
public class Test {
    public static void main(String[] args) {
        System.out.println("Hello, World!")
    }
}
```

Now, run the linter:

```bash
target/release/lintymclintface -l java -f Test.java
```

Expected JSON output:

```json
[
  {
    "line": 3,
    "column": 44,
    "message": "Missing ;"
  }
]
```

**Example: Linting an R file**

Create a dummy R file with a syntax error (e.g., `test_r_error.R`):

```R
my_var <- c(1, 2, 3
```

Now, run the linter:

```bash
target/release/lintymclintface -l r -f test_r_error.R
```

Expected JSON output:

```json
[
  {
    "line": 1,
    "column": 16,
    "message": "Missing )"
  }
]
```

**Controlling Logging Verbosity (CLI)**

By default, the CLI output is concise. To see more detailed debug information, you can set the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug target/release/lintymclintface -l python -f test_python_error.py
```

### Running Tests

To run the unit tests for `lintymclintface`, which include checks for Python, Java, and R files (both working and intentionally failing cases), use the following command:

```bash
cargo test
```

To see detailed output from the tests, including which files are being tested, use:

```bash
cargo test -- --nocapture
```

### Web Service (REST API)


`lintymclintface` can also run as a web service, accepting linting requests via HTTP POST.

1.  **Start the service**:
```bash
target/release/lintymclintface --service &
```
    This will start the service in the background on `http://127.0.0.1:8080`. You can observe logging output in the terminal where you ran this command.

2.  **Linting via API (using cURL)**:

    Send a POST request to the `/lint` endpoint with `language` and `code` in the JSON body.

    **Python Example:**
    ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"language": "python", "code": "def my_function():\n    print(\"Hello, world!\""}' http://127.0.0.1:8080/lint
    ```

    **Java Example:**
    ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"language": "java", "code": "public class MyClass { public static void main(String[] args) { System.out.println(\"Hello\") } }"}' http://127.0.0.1:8080/lint
    ```

    **R Example:**
    ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"language": "r", "code": "my_data <- data.frame(x = 1:3, y = c(\"a\", \"b\", \"c\")"}' http://127.0.0.1:8080/lint
    ```

3.  **Accessing Prometheus Metrics**:

    The service exposes Prometheus-compatible metrics on the `/metrics` endpoint:
    ```bash
    curl http://127.0.0.1:8080/metrics
    ```
    You will see metrics like `lint_requests_total`, `lint_requests_by_language`, `lint_duration_seconds`, and `lint_errors_total`.

### Examples Directory

The `examples/` directory contains shell scripts to demonstrate linting entire popular open-source repositories for each supported language. These scripts will clone the respective repositories (if not already present) and then run `lintymclintface` over all relevant source files.

To run them, navigate to the `examples/` directory and execute the desired script:

```bash
cd examples/
./lint_python_repo.sh
./lint_java_repo.sh
./lint_r_repo.sh
```

## License

This project is licensed under the [MIT License](LICENSE).
