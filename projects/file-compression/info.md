Here’s a line-by-line breakdown of the Rust code you provided, with comments and explanations for each part, especially focusing on the `unwrap`, `GzEncoder`, and `flate2` concepts.

---

```rust
// Import the `flate2` crate, which provides compression functionality.
extern crate flate2;

// Import specific types from the flate2 crate
use flate2::write::GzEncoder;  // `GzEncoder` is used to write compressed data in gzip format.
use flate2::Compression;       // `Compression` specifies the compression level.

// Import standard library modules and types.
use std::env::args;            // `args` retrieves command-line arguments passed to the program.
use std::fs::File;             // `File` handles reading from and writing to files.
use std::io::copy;             // `copy` function copies data from one stream to another.
use std::io::BufReader;        // `BufReader` buffers reads, improving efficiency.
use std::time::Instant;        // `Instant` is used to measure elapsed time.

fn main() {
    // Check if the correct number of arguments is passed.
    if args().len() != 3 {
        eprintln!("you need to pass file path and compressed File Name");
        return;
    }

    // Create a buffered reader for the input file specified in the command line arguments.
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    // Open or create the output file for the compressed data, as specified in the arguments.
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // Initialize the gzip encoder with the output file and default compression level.
    let encoder = GzEncoder::new(output, Compression::default());

    // Start the timer to measure the time taken for the compression process.
    let start = Instant::now();

    // Copy the contents of `input` to the `encoder`, compressing as it goes.
    copy(&mut input, &mut encoder).unwrap();

    // Finish the compression process and finalize the output file.
    let output = encoder.finish().unwrap();

    // Print the original file size by accessing metadata.
    println!("source len: {:?}", input.get_ref().metadata().unwrap().len());

    // Print the compressed file size from the output's metadata.
    println!("Target len: {:?}", output.metadata().unwrap().len());

    // Print the time taken for the entire process.
    println!("Time Elapsed {:?}", start.elapsed());
}
```

### Detailed Explanation

1. **Imports**:

   - `flate2` crate: Contains functionality for handling compressed data (gzip format). The `flate2::write::GzEncoder` structure handles the compression, while `flate2::Compression` allows choosing the compression level.
   - Standard libraries like `env::args`, `fs::File`, `io::copy`, and `time::Instant` help with file handling, I/O operations, and tracking time.

2. **Command-Line Arguments**:

   - `args()`: Collects command-line arguments. The program expects two arguments: the source file path and the desired compressed output file name.
   - `args().nth(n).unwrap()`: Retrieves the argument at index `n`. `unwrap()` here forces the program to panic if the argument is not present, which ensures the file paths are always provided.

3. **Buffered Reader and File Handling**:

   - `BufReader::new(File::open(...).unwrap())`: Opens the source file and wraps it in a `BufReader` for efficient reading.
   - `File::create(...)`: Creates or opens the output file for compressed data.

4. **Compression**:

   - `GzEncoder::new(output, Compression::default())`: Initializes `GzEncoder` to compress data written to the `output` file. `Compression::default()` sets the compression level.
   - `copy(&mut input, &mut encoder)`: Reads from `input` and writes compressed data to `encoder`.
   - `encoder.finish()`: Completes the compression, returning the output file handler.

5. **Metadata and Timing**:
   - `input.get_ref().metadata().unwrap().len()`: Retrieves the original file size.
   - `output.metadata().unwrap().len()`: Retrieves the compressed file size.
   - `start.elapsed()`: Tracks and displays the time taken for the entire compression process.

### Concepts to Understand

- **`unwrap()`**: Common in Rust, `unwrap()` is a shortcut to handle `Result` types. If an operation is successful (`Ok`), `unwrap()` returns the value; if it fails (`Err`), the program panics with an error message.
- **`GzEncoder`**: Part of `flate2`, this struct writes gzip-compressed data to the specified output.
- **`BufReader`**: Buffers reads, which can make I/O operations faster by reducing the number of actual system calls needed for reading files.

This code example demonstrates file I/O, error handling, compression, and time measurement in Rust, all valuable when working on larger projects.
