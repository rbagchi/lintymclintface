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

**Running on an Arbitrary Host Port:**

To map the container's port 8080 to a different port on your host machine (e.g., 9000), modify the `-p` flag:

```bash
docker run -d -p 9000:8080 --name lintymclintface-container lintymclintface-service
```

**Changing the Container's Internal Listening Port (Advanced):**

The `lintymclintface` application inside the container listens on port 8080 by default. You can change this by overriding the `CMD` in the `docker run` command and specifying the `--port` argument or `LINT_SERVER_PORT` environment variable. Remember to adjust the host port mapping accordingly.

Example (container listens on 8081, mapped to host 9000):

```bash
docker run -d -p 9000:8081 --name lintymclintface-container lintymclintface-service \
  ./lintymclintface --service --port 8081
```

Or using an environment variable:

```bash
docker run -d -p 9000:8081 --name lintymclintface-container lintymclintface-service \
  -e LINT_SERVER_PORT=8081
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
