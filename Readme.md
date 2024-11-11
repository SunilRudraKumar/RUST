
---

# File Statistics Tool

A command-line application built in Rust to analyze the contents of a text file and provide useful statistics, such as the number of lines, words, and characters. This project helps solidify skills in file handling, string manipulation, and error handling in Rust.

## Table of Contents
- [Project Overview](#project-overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Example](#example)
- [Implementation Details](#implementation-details)
- [Future Enhancements](#future-enhancements)
- [Contributing](#contributing)
- [License](#license)

## Project Overview

The File Statistics Tool is a simple CLI application that reads a text file and provides basic statistics:
- **Line Count**: Total number of lines in the file.
- **Word Count**: Total number of words in the file.
- **Character Count**: Total number of characters, including whitespace.

This project is part of a series of CLI application exercises to learn foundational programming concepts in Rust.

## Features

- **Efficient File Handling**: Reads files line-by-line for optimal memory usage.
- **Detailed Error Handling**: Provides clear messages if the file cannot be read or doesn’t exist.
- **Statistics Summary**: Outputs total lines, words, and characters.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone  https://github.com/SunilRudraKumar/Rust-file-status.git
   cd Rust-file-status
   ```

2. **Build the Project**:
   ```bash
   cargo build --release
   ```

3. **Run the Program**:
   ```bash
   cargo run -- <filename>
   ```

## Usage

The File Statistics Tool expects a text file as input. Run the program with the file path as an argument:

```bash
./file_statistics_tool <filename>
```

Replace `<filename>` with the path to the file you want to analyze.

## Example

### Sample Command:
```bash
./file_statistics_tool sample.txt
```

### Sample Output:
```plaintext
File: sample.txt
Lines: 100
Words: 520
Characters: 3020
```

## Implementation Details

- **File Reading**: The tool uses Rust’s `std::fs::File` and `std::io::BufReader` to read the file efficiently.
- **Line, Word, and Character Counting**:
  - **Lines**: Each line read increments the line count.
  - **Words**: Each line is split by whitespace to count words.
  - **Characters**: Each line’s characters are counted, including spaces.
- **Error Handling**: If the file is not found or is inaccessible, a friendly error message is displayed.

### Code Snippet for File Reading
```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
```

## Future Enhancements

- **Extended Statistics**: Add options for more advanced statistics, like paragraph count, average word length, and sentence count.
- **Output Formatting**: Allow output in JSON or other structured formats.
- **Additional Language Support**: Add handling for non-ASCII characters and custom delimiters.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with any new features, optimizations, or fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---
