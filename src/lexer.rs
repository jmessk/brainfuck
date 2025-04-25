pub enum Token {
    IncrementValue,   // '+'
    DecrementValue,   // '-'
    IncrementPointer, // '>'
    DecrementPointer, // '<'
    LoopBegin,        // '['
    LoopEnd,          // ']'
    Input,            // ','
    Output,           // '.'
}

pub struct TokenStream {
    pub tokens: Vec<Token>,
}

pub struct Lexer;

impl Lexer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Lexer {}
    }

    pub fn tokenize(&self, input: &str) -> anyhow::Result<TokenStream> {
        let mut tokens = Vec::with_capacity(input.len());

        for c in input.chars() {
            match c {
                '+' => tokens.push(Token::IncrementValue),
                '-' => tokens.push(Token::DecrementValue),
                '>' => tokens.push(Token::IncrementPointer),
                '<' => tokens.push(Token::DecrementPointer),
                '[' => tokens.push(Token::LoopBegin),
                ']' => tokens.push(Token::LoopEnd),
                '.' => tokens.push(Token::Output),
                ',' => tokens.push(Token::Input),
                _ => {}
            }
        }

        Ok(TokenStream { tokens })
    }
}
