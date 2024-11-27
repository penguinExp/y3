use std::{io, path::Path};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        eprintln!("[Error] File not found - {}", file_path);
        return Ok(());
    }

    Ok(())
}

fn print_help() {
    const TEXT: &str = r#"
    Usage:
        y3 <file_path>

    Description:

    This program reads a file, extracts words, and prints each word along with its position.

    Example:
    
    y3 dummy_text.txt

    "#;

    println!("{TEXT}");
}
