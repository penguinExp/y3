use std::io;
use y3::reader::Reader;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let dir_path = &args[1];

    let mut reader = Reader::new(dir_path);

    reader.get_files(dir_path)?;

    println!("Fetched {} files!", reader.paths().len());

    println!("----------");

    for p in reader.paths() {
        println!("{:?}", p);
    }

    // if !Path::new(file_path).exists() {
    //     eprintln!("[Error] File not found - {}", file_path);
    //     return Ok(());
    // }

    // let mut tokenizer = Tokenizer::new();

    // tokenizer.tokenize(file_path)?;

    // for token in tokenizer.tokens() {
    //     println!("{:?}", token);
    // }

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
