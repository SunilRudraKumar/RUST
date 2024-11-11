// Import the `flate2` crate, which provides compression functionality.
extern crate flate2;

// Import specific types from the flate2 crate
use flate2::write::GzEncoder; // `GzEncoder` is used to write compressed data in gzip format.
use flate2::Compression; // `Compression` specifies the compression level.

// Import standard library modules and types.
use std::env::args; // `args` retrieves command-line arguments passed to the program.
use std::fs::File; // `File` handles reading from and writing to files.
use std::io::copy; // `copy` function copies data from one stream to another.
use std::io::BufReader; // `BufReader` buffers reads, improving efficiency.
use std::time::Instant; // `Instant` is used to measure elapsed time.

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
    let mut encoder = GzEncoder::new(output, Compression::default());

    // Start the timer to measure the time taken for the compression process.
    let start = Instant::now();

    // Copy the contents of `input` to the `encoder`, compressing as it goes.
    copy(&mut input, &mut encoder).unwrap();

    // Finish the compression process and finalize the output file.
    let output = encoder.finish().unwrap();

    // Print the original file size by accessing metadata.
    println!(
        "source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    // Print the compressed file size from the output's metadata.
    println!("Target len: {:?}", output.metadata().unwrap().len());

    // Print the time taken for the entire process.
    println!("Time Elapsed {:?}", start.elapsed());
}
