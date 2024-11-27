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
/// A custom tokenizer which reads through the input file and parses words
/// to be spell checked as [Token]'s
///
#[derive(Debug)]
pub struct Tokenizer {
    ///
    /// List of parsed tokens from the input file
    ///
    tokens: Vec<Token>,
}

impl Tokenizer {
    ///
    /// Getter to read the list of parsed [Token]'s
    ///
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}
