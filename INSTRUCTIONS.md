# LLM-Assisted Code Development with Pre-Linting

To improve the efficiency of our development workflow, we use a custom linter to catch syntax errors *before* running full build processes. This provides faster feedback and reduces unnecessary round trips for simple syntax issues across various programming languages.

As an LLM assistant, your role is to use this linter to validate code changes and provide targeted feedback to the user.

## Workflow

1.  **Run the Linter:** Before proceeding with compilation or execution, always run the linter on the modified file(s).

    ```bash
    target/release/lintymclintface -l <language> -f <file_path>
    ```

    *   `<language>`: Specify the programming language. Supported options are `python`, `java`, or `r`.
    *   `<file_path>`: The path to the file you want to lint.

2.  **Analyze the Output:** The linter produces JSON output. An empty array `[]` means no errors were found. If there are errors, the JSON will contain a list of objects, each describing an error:

    ```json
    [
      {
        "line": 3,
        "column": 44,
        "message": "Missing ;"
      }
    ]
    ```

3.  **Provide Feedback:** If the linter finds errors, use the line and column numbers and the error message to provide a precise fix to the user. Do not proceed to the next development step (e.g., compilation, testing) until the linter reports no errors.

4.  **Proceed with Build/Execution:** Once the linter passes (i.e., returns an empty array), you can proceed with the language-specific build, compilation, or execution steps (e.g., Bazel for Java, `pytest` for Python, `R CMD check` for R).

## Example Scenarios

### Java Example

Let's say a user modifies `src/main/java/com/example/Greeter.java` and introduces a syntax error:

**User:** "I've updated the greeter to be more enthusiastic."

**Modified `Greeter.java`:**
```java
package com.example;

public class Greeter {
    public String greet(String name) {
        return "Hello, " + name + "!!"
    }
}
```

**Your Workflow:**

1.  **Run the linter:**

    ```bash
    target/release/lintymclintface -l java -f src/main/java/com/example/Greeter.java
    ```

2.  **Analyze the output:**

    ```json
    [
      {
        "line": 5,
        "column": 35,
        "message": "Missing ;"
      }
    ]
    ```

3.  **Provide feedback to the user:**

    "I found a syntax error in `src/main/java/com/example/Greeter.java`. It looks like you're missing a semicolon on line 5. Here's the corrected line:

    ```java
    return "Hello, " + name + "!!";
    ```

    Once you've made that change, I'll proceed with the build."

4.  **Proceed with build:** After the user confirms the fix, and the linter passes, proceed with the Java build (e.g., using Bazel):

    ```bash
    bazel build //...
    ```

### Python Example

Let's say a user modifies a Python file (e.g., `my_script.py`) and introduces a syntax error:

**User:** "I've added a new feature to the script."

**Modified `my_script.py`:**
```python
def calculate_sum(a, b):
    result = a + b
    print("The sum is: " result)
```

**Your Workflow:**

1.  **Run the linter:**

    ```bash
    target/release/lintymclintface -l python -f my_script.py
    ```

2.  **Analyze the output:**

    ```json
    [
      {
        "line": 3,
        "column": 24,
        "message": "Syntax error near 'result)'"
      }
    ]
    ```

3.  **Provide feedback to the user:**

    "I found a syntax error in `my_script.py`. It looks like you're missing a comma between the string and the variable `result` on line 3. Here's the corrected line:

    ```python
    print("The sum is: ", result)
    ```

    Once you've made that change, I'll proceed with testing."

### R Example

Let's say a user modifies an R script (e.g., `my_analysis.R`) and introduces a syntax error:

**User:** "I've updated the data analysis script."

**Modified `my_analysis.R`:**
```R
data_frame <- data.frame(x = 1:5, y = c("A", "B", "C", "D", "E"
```

**Your Workflow:**

1.  **Run the linter:**

    ```bash
    target/release/lintymclintface -l r -f my_analysis.R
    ```

2.  **Analyze the output:**

    ```json
    [
      {
        "line": 1,
        "column": 50,
        "message": "Missing )"
      }
    ]
    ```

3.  **Provide feedback to the user:**

    "I found a syntax error in `my_analysis.R`. It looks like you're missing a closing parenthesis on line 1. Here's the corrected line:

    ```R
    data_frame <- data.frame(x = 1:5, y = c("A", "B", "C", "D", "E"))
    ```

    Once you've made that change, I'll proceed with the analysis."