use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

///
/// Struct to represent the position of the [Token] in the input file
///
#[derive(Debug)]
pub struct Position {
    ///
    /// Starting position of the token in the input file
    ///
    start: usize,

    ///
    /// Ending position of the token in the input file
    ///
    end: usize,

    ///
    /// Line number of the token in the input file
    ///
    line_no: usize,
}

impl Position {
    ///
    /// Getter to read the [start] position offset for the token
    ///
    pub fn start(&self) -> usize {
        self.start
    }

    ///
    /// Getter to read the [end] position offset for the token
    ///
    pub fn end(&self) -> usize {
        self.end
    }

    ///
    /// Getter to read the [line_no] of the token
    ///
    pub fn line_no(&self) -> usize {
        self.line_no
    }
}

///
/// Struct representing word parsed from input file to be spell checked
///
/// # Example
/// ```rs
/// let token = Token::new("word", 0, 3, 1);
/// ```
///
#[derive(Debug)]
pub struct Token {
    ///
    /// Parsed word from the input file
    ///
    word: String,

    ///
    /// Position offset of the token in the input file.
    ///
    /// It's used to show position of the misspelled word to the user
    ///
    position: Position,
}

impl Token {
    pub fn new(word: &str, start: usize, end: usize, line_no: usize) -> Self {
        Self {
            word: word.to_string(),
            position: Position {
                start,
                end,
                line_no,
            },
        }
    }

    ///
    /// Getter to read the parsed `word`
    ///
    pub fn word(&self) -> &str {
        &self.word
    }

    ///
    /// Getter to read the parsed [Position]
    ///
    pub fn position(&self) -> &Position {
        &self.position
    }
}

///
/// A structure holding [Regex] patterns to be used while parsing
///
#[derive(Debug)]
struct Patterns {
    ///
    /// List of [Regex] patterns to be ignored while parsing.
    ///
    /// Fallowing types are ignored,
    /// - Link's, URL's,
    /// - File Paths
    /// - Direct numbers like "1234"
    /// - Email like patterns
    /// - Regular Expressions
    ///
    ignore_patterns: Vec<Regex>,

    ///
    /// A [Regex] pattern to match against potential tokens
    ///
    word_pattern: Regex,

    ///
    /// A [Regex] pattern to split words to form tokens
    ///
    /// Useful while separating words like,
    /// - `snake_case` to ["snake", "case"]
    /// - `Get-Item` to ["Get", "Item"]
    /// - `run—but` to ["run", "but"]
    ///
    split_pattern: Regex,
}

impl Patterns {
    fn new() -> Self {
        Self {
            ignore_patterns: vec![
                // URLs
                Regex::new(r"https?://\S+").unwrap(),
                // File paths
                Regex::new(r"[\w\-\.]+(/[\w\-\.]+)+").unwrap(),
                // Pure numbers
                Regex::new(r"\b\d+\b").unwrap(),
                // Regex patterns
                Regex::new(r"\\[a-zA-Z]+[{[^()]+}]*").unwrap(),
                // Email-like patterns
                Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap(),
            ],
            // potential tokens
            word_pattern: Regex::new(r"[a-zA-Z]+[0-9]*[a-zA-Z]*").unwrap(),
            // split formats like -, _, etc.
            split_pattern: Regex::new(r"[ _\-—]").unwrap(),
        }
    }
}

///
/// A custom tokenizer which reads through the input file and parses words
/// to be spell checked as [Token]'s
///
#[derive(Debug)]
pub struct Tokenizer {
    ///
    /// List of parsed tokens from the input file
    ///
    tokens: Vec<Token>,

    ///
    /// Set of [Regex] patterns used for parsing tokens
    ///
    patterns: Patterns,
}

impl Tokenizer {
    ///
    /// Getter to read the list of parsed [Token]'s
    ///
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    ///
    /// Create an instance of [Tokenizer]
    ///
    /// # Example
    ///
    /// ```rs
    ///  let mut tokenizer = Tokenizer::new();
    ///  assert_eq!(tokenizer.tokens().len(), 0 as usize);
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            patterns: Patterns::new(),
        }
    }

    ///
    /// Clear the list of parsed [Token]'s
    ///
    /// ## NOTE
    ///
    /// It has no effect on the allocated memory for the [Vec].
    ///
    /// This saves the overhead of reallocating the memory again, it
    /// simply uses pre-allocated memory for upcoming tokens.
    ///
    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    pub fn tokenize(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for (line_no, line) in reader.lines().enumerate() {
            let line = line?;
            let mut pos: usize = 0;

            // split lines
            for chunk in self.patterns.split_pattern.split(&line) {}
        }

        Ok(())
    }
}
