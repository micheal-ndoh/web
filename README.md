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
