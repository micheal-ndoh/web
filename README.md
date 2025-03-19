# Rust Server

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


Based on your provided workspace structure and code, I’ll analyze the issues and suggest possible fixes. Here’s a breakdown of the problems and solutions:

---

### **1. Workspace Configuration**
Your `Cargo.toml` workspace file looks correct, but ensure all member projects (`file-uploader`, `file-compression`, `upload-endpoint`) are in the same parent directory and properly linked.

#### **Fix**:
- Verify the directory structure:
  ```
  workspace/
  ├── Cargo.toml
  ├── file-uploader/
  ├── file-compression/
  └── upload-endpoint/
  ```
- Ensure each member project has its own `Cargo.toml` and `src` directory.

---

### **2. `upload-endpoint` Issues**
#### **Problem 1: File Path Handling**
- The `upload` function writes files to `../files/`, which might cause issues if the directory doesn’t exist or the path is incorrect.

#### **Fix**:
- Create the `files` directory programmatically or ensure it exists before running the server.
  ```rust
  use std::fs;

  pub async fn upload(mut multipart: Multipart) {
      fs::create_dir_all("../files").expect("Failed to create 'files' directory");
      // Rest of the code...
  }
  ```

#### **Problem 2: Error Handling**
- The `unwrap()` calls can cause panics if something goes wrong (e.g., missing file name or failed I/O).

#### **Fix**:
- Replace `unwrap()` with proper error handling:
  ```rust
  pub async fn upload(mut multipart: Multipart) -> Result<(), Box<dyn std::error::Error>> {
      fs::create_dir_all("../files")?;
      while let Some(field) = multipart.next_field().await? {
          if field.name()? != "fileupload" {
              continue;
          }
          let file_name = field.file_name().ok_or("Missing file name")?;
          let file_path = format!("../files/{}", file_name);
          let data = field.bytes().await?;
          let mut file_handle = File::create(file_path)?;
          file_handle.write_all(&data)?;
      }
      Ok(())
  }
  ```

---

### **3. `file-uploader` Issues**
#### **Problem 1: Dependency on `upload-endpoint`**
- The `file-uploader` project depends on `upload-endpoint`, but it’s not clear why. This dependency might be unnecessary.

#### **Fix**:
- Remove the dependency from `file-uploader/Cargo.toml`:
  ```toml
  [dependencies]
  structopt = "0.3"
  tokio = { version = "1", features = ["full"] }
  clap = { version = "4", features = ["derive"] }
  reqwest = { version = "0.12.14", features = ["multipart"] }
  ```

#### **Problem 2: StructOpt Deprecation**
- `StructOpt` is deprecated in favor of `clap`.

#### **Fix**:
- Replace `StructOpt` with `clap`:
  ```rust
  use clap::Parser;

  #[derive(Debug, Parser)]
  pub struct Opt {
      #[clap(parse(from_os_str))]
      infile: PathBuf,

      #[clap(short, long, parse(from_os_str))]
      outfile: Option<PathBuf>,
  }

  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error>> {
      let opts = Opt::parse();
      // Rest of the code...
  }
  ```

---

### **4. `file-compression` Issues**
#### **Problem 1: Unused Dependency**
- The `rust_file_compression` dependency is unused and might cause confusion.

#### **Fix**:
- Remove the unused dependency from `file-compression/Cargo.toml`:
  ```toml
  [dependencies]
  flate2 = { version = "1.0.17", features = ["full"], default-features = false }
  ```

#### **Problem 2: Error Handling**
- The `compress_file` function doesn’t handle errors gracefully.

#### **Fix**:
- Improve error handling:
  ```rust
  pub fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
      let mut input_file = File::open(input_path)?;
      let output_file = File::create(output_path)?;
      let mut encoder = GzEncoder::new(output_file, Compression::default());
      let mut buffer = Vec::new();
      input_file.read_to_end(&mut buffer)?;
      encoder.write_all(&buffer)?;
      encoder.finish()?;
      Ok(())
  }
  ```

---

### **5. General Improvements**
#### **Problem 1: Workspace Build**
- Ensure all projects build correctly within the workspace.

#### **Fix**:
- Run `cargo build --workspace` to verify all projects compile.

#### **Problem 2: Documentation**
- Add comments and documentation to improve code readability.

#### **Fix**:
- Add doc comments to functions and modules:
  ```rust
  /// Compresses a file using Gzip.
  ///
  /// # Arguments
  /// * `input_path` - Path to the input file.
  /// * `output_path` - Path to the output compressed file.
  ///
  /// # Returns
  /// `io::Result<()>` - Result indicating success or failure.
  pub fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
      // Function implementation...
  }
  ```

---

### **Final Notes**
- Test each project individually to ensure they work as expected.
- Use `cargo clippy` and `cargo fmt` to enforce code quality and style.
- Consider adding unit tests for critical functionality.

Let me know if you need further assistance!