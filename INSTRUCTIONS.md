# LLM-Assisted Java Development with Pre-Linting

To improve the efficiency of our development workflow, we use a custom linter to catch Java syntax errors *before* running the full Bazel build. This provides faster feedback and reduces unnecessary round trips for simple syntax issues.

As an LLM assistant, your role is to use this linter to validate Java code changes and provide targeted feedback to the user.

## Workflow

1.  **Run the Linter:** Before running a Bazel build, always run the linter on the modified Java file(s).

    ```bash
    ./target/debug/java-syntax <path_to_java_file>
    ```

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

3.  **Provide Feedback:** If the linter finds errors, use the line and column numbers and the error message to provide a precise fix to the user. Do not proceed to the Bazel build until the linter reports no errors.

4.  **Run the Bazel Build:** Once the linter passes (i.e., returns an empty array), you can proceed with the Bazel build to check for compilation and other errors.

    ```bash
    bazel build //...
    ```

## Example Scenario

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
    ./target/debug/java-syntax src/main/java/com/example/Greeter.java
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

    Once you've made that change, I'll run the full build."

4.  **Run Bazel build:** After the user confirms the fix, and the linter passes, run the Bazel build:

    ```bash
    bazel build //...
    ```

By following this process, you can provide a much faster and more efficient development experience.
