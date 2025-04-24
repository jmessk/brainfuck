use bf::{Context, Lexer, Parser};
use std::path::PathBuf;

struct Runtime {
    lexer: Lexer,
    parser: Parser,
    context: Context,
}

impl Runtime {
    fn with_mem_size(mem_size: usize) -> Self {
        Runtime {
            lexer: Lexer::new(),
            parser: Parser::new(),
            context: Context::with_mem_size(mem_size),
        }
    }

    fn eval(&mut self, input: &str) -> anyhow::Result<()> {
        let token_stream = self.lexer.tokenize(input)?;
        let parser = self.parser.parse(token_stream)?;

        self.context.eval(&parser)?;

        Ok(())
    }
}

#[derive(clap::Parser)]
struct Args {
    file: PathBuf,

    #[arg(long, short, default_value = "10000")]
    mem_size: usize,
}

fn main() {
    use clap::Parser as _;
    let args = Args::parse();

    let mut runtime = Runtime::with_mem_size(args.mem_size);

    let input = std::fs::read_to_string(args.file).expect("Failed to read file");

    runtime.eval(&input).expect("Failed to eval code");
}
