# Rust Server

## A series of tasks to learn how to set up a Rust web server

### Task 1 (Monday): "Hello World Web Server"

    - Goal: Set up a minimal Rust web server.
    - Description:
        - Create a new Rust project that uses a web framework (e.g., Axum).
        - Implement a basic endpoint (such as GET /) that returns “Hello, World!”
        This task introduces basic project setup, dependency management, and the fundamentals of HTTP servers in Rust.
    - Repository Name: rust-hello-server

### Task 2 (Tuesday): "Implementing a File Upload Endpoint"

    - Goal: Build a REST API endpoint to handle file uploads.
    - Description:
        - Extend the server from Task 1 (or start a new repo) to accept file uploads via multipart/form-data.
        - Write the uploaded file to disk and respond with a success message.
        - This teaches handling HTTP POST requests, multipart parsing, and basic file I/O.
    - Repository Name: rust-file-upload-endpoint
  
### Task 3 (Wednesday): "Building a Basic CLI Client for Uploads"

    - Goal: Develop a CLI client in Rust that uploads files.
    - Description:
        - Create a command-line tool that takes a file path as an argument.
        - Use an HTTP client (such as reqwest) to send the file to the upload endpoint from Task 2.
        - Focus on argument parsing (using Clap or StructOpt) and HTTP client integration.
    - Repository Name:` rust-cli-file-uploader`

- Note:  Use a workspaces to link both task 2 and 3 from differents directories

### Task 4 (Thursday): "Standalone File Compression Function"

    - Goal: Implement a function that compresses files using a Rust compression library.
    - Description:
        - Use a library like flate2 or brotli to compress a given file.
        - Write this as a self-contained function or small program that reads an input file and writes a compressed output file.
          This task introduces external libraries, file processing, and basic algorithm integration.
    - Repository Name: rust-file-compression.

### Task 5 (Monday): "Batch File Upload Support"

    - Goal: Enhance the file upload endpoint to support multiple files.
    - Description:
      - Modify the file upload endpoint so it can accept and process several files in one HTTP request.
      - Ensure each file is processed (e.g., stored on disk) and report back an aggregated result.

### Task 6: Interactive CLI for Batch Uploads

    - Goal: Upgrade the CLI to handle multiple file uploads interactively.
    - Description:
        Extend the CLI from Task 3 to allow the user to specify multiple files (or a directory).
        Add simple progress output (even if just textual status updates) as files are being uploaded.

### Task 7: Supplying Compression Parameters

    - Goal: Allow the client to select compression techniques and levels.
    - Description:
        Update both the CLI and server to accept compression options (for example, via command-line flags or HTTP parameters).
        Adjust the compression function (from Task 4) to take these parameters into account.

### Task 8: Asynchronous Compression Handling

    - Goal: Offload compression tasks so that the server remains responsive.
    - Description:
        Modify the server to process compression in an asynchronous manner (using Tokio or async-std).
        When a file is uploaded, immediately return a unique task ID, and process the compression in the background.

### Task 9: "Status Endpoint for Compression Tasks"

    - Goal: Provide clients with real-time status updates for compression jobs.
    - Description:
        Develop a REST API endpoint where a client can query the status (e.g., pending, processing, completed, failed) of a given task using its ID.

### Task 10: Implementing a Task Queue

    - Goal: Build an internal mechanism to manage and track asynchronous tasks.
    - Description:
        Design a simple task queue or in-memory store that keeps track of each compression job’s state and progress.
        Assign unique IDs and update statuses as jobs move through the pipeline.

### Task 11:Robust Error Handling and Retry Logic

    - Goal: Ensure reliability by handling errors gracefully.
    - Description:
        Enhance the asynchronous processing (from Task 8) with robust error detection.
        Implement retry logic for failed tasks and clear error reporting to the status endpoint.

### Task 12: Performance Testing and Benchmarking

    - Goal: Validate and improve system performance.
    - Description:
        Write tests and benchmarks to simulate concurrent uploads and compression tasks.
        Measure performance, identify bottlenecks, and document potential improvements.

To run first:

Run the database

```rs
 docker run -it --name compression_db -e POSTGRES_USER=micheal -e POSTGRES_PASSWORD=nemory09 -e POSTGRES_DB=compression_tasks -p 5432:5432 -d postgres
```

Then exec to it and run the migrations

```rs
docker exec -it compression_db psql -U micheal -d compression_tasks
```

Then run the migrations

```rest
sqlx migrate run 
```

Serve the endpoint:

```rust
cd upload-endpoint
cargo run -p upload-endpoint
```

Upload the files to the endpoint

```bash
cargo run -p file-uploader -- /path/to/file1 /path/to/file2
```

to compress the files

```rust
  curl -X POST http://localhost:3000/compressor/compress
```

or

```rust
curl -v -X POST http://localhost:3000/compressor/compress   
```

To get the status of the compression task use

```rust
curl http://localhost:3000/check/<task_id>
```

To get swagger documentation

```rust
http://localhost:3000/swagger-ui
```
