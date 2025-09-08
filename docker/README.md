# Docker Instructions for lintymclintface

This directory contains the necessary files to build and run the `lintymclintface` service within a Docker container.

## Build the Docker Image

Navigate to the root of the `lintymclintface` project and run the following command to build the Docker image. This might take a few minutes as it compiles the Rust application.

```bash
docker build -t lintymclintface-service -f docker/Dockerfile .
```

## Run the Docker Container

Once the image is built, you can run the service in a container. The `-p 8080:8080` flag maps port 8080 of your host machine to port 8080 inside the container, allowing you to access the service.

```bash
docker run -d -p 8080:8080 --name lintymclintface-container lintymclintface-service
```

To observe logs from the container, use `docker logs lintymclintface-container`. Note that ANSI color output has been disabled in the application's logging for better compatibility with `docker logs`.

To stop the container:

```bash
docker stop lintymclintface-container
```

To remove the container:

```bash
docker rm lintymclintface-container
```

## Test the Service

After the container is running, you can test the service using `curl` or the provided `bin/call_linter_service.sh` script.

### Using `bin/call_linter_service.sh`

Make sure the `bin/call_linter_service.sh` script is executable (`chmod +x bin/call_linter_service.sh`). Then, you can call it like this:

```bash
./bin/call_linter_service.sh java src/main/java/com/example/Main.java
```

(Replace `java` and `src/main/java/com/example/Main.java` with your desired language and file path.)

### Using `curl` directly

First, create a temporary file with some code (e.g., `temp.java`):

```java
// temp.java
public class MyClass {
    public static void main(String[] args) {
        System.out.println("Hello, Docker!")
    }
}
```

Then, send a POST request to the service:

```bash
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{"language": "java", "code": "$(cat temp.java)"}' \
     http://localhost:8080/lint
```

This will send the content of `temp.java` to the linter service running in the Docker container and return any found syntax errors.
