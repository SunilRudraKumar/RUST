use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let zero = &args[0];

    let file_path = &args[1];

    println!("In zero {zero}");

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");
    println!("The contents of the file:\n{contents}");

    // Call the line_count and word_count functions
    let line_count_result = line_count(&contents);
    let word_count_result = word_count(&contents);
    let character_count_result = char_count(&contents);
    println!("Line count: {line_count_result}");
    println!("Word count: {word_count_result}");
    println!("Character count: {character_count_result}");
}

fn line_count(content: &str) -> u32 {
    let mut count = 0;

    for c in content.chars() {
        if c == '\n' {
            count += 1;
        }
    }

    // If content doesn't end with a newline, count the last line
    if !content.is_empty() && !content.ends_with('\n') {
        count += 1;
    }

    count
}

fn word_count(content: &str) -> u32 {
    let mut in_word = false;
    let mut count = 0;

    for c in content.chars() {
        if c.is_whitespace() {
            in_word = false;
        } else {
            if !in_word {
                count += 1;
            }
            in_word = true;
        }
    }

    count
}

fn char_count(content: &str) -> u32 {
    let mut count = 0;

    for _c in content.chars() {
        count += 1;
    }

    count
}
