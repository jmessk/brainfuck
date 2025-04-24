# Brainfuck Interpreter

This is a simple Brainfuck interpreter written in Rust.

## Usage

To run the interpreter, you need to have Rust installed. You can install Rust [here](https://www.rust-lang.org/tools/install).

```bash
cargo run --release -- <path_to_file>

# e.g. 
cargo run --release -- ./examples/bf/test2.bf
```

command line arguments:

- `--help`, `-h`: Show help message.
- `--mem-size`, `-m`: Set memory size (default: 10,000 bytes).

## Implement your own interpreter

If you want to implement your own Brainfuck interpreter, you can use the provided `Context`, `Lexer`, and `Parser` structs. Here's a simple example of how to use them:

```rust
use bf::{Context, Lexer, Parser};

struct Runtime {
    lexer: Lexer,
    parser: Parser,
    context: Context,
}

impl Runtime {
    fn new() -> Self {
        Runtime {
            lexer: Lexer::new(),
            parser: Parser::new(),
            context: Context::default(),
        }
    }

    fn eval(&mut self, input: &str) -> anyhow::Result<()> {
        let token_stream = self.lexer.tokenize(input)?;
        let ast = self.parser.parse(token_stream)?;

        self.context.eval(&ast)?;

        Ok(())
    }
}
```
