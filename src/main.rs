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
        let ast = self.parser.parse(token_stream)?;

        self.context.eval(&ast)?;

        Ok(())
    }
}

#[derive(clap::Parser)]
struct Args {
    file: PathBuf,

    #[arg(long, short, default_value_t = 10_000)]
    mem_size: usize,
}

fn main() -> anyhow::Result<()> {
    use clap::Parser as _;
    let args = Args::parse();

    let input = std::fs::read_to_string(args.file)?;

    Runtime::with_mem_size(args.mem_size).eval(&input)
}
