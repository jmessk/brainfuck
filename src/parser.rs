use crate::lexer::{Token, TokenStream};

pub enum AstNode {
    IncrementValue,
    DecrementValue,
    IncrementPointer,
    DecrementPointer,
    Loop(Ast),
    Input,
    Output,
}

pub struct Ast {
    pub nodes: Vec<AstNode>,
}

pub struct Parser {}

impl Parser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, token_stream: TokenStream) -> anyhow::Result<Ast> {
        let mut stack = Vec::new();
        let mut current_node = Vec::new();

        for token in &token_stream.tokens {
            match token {
                Token::IncrementValue => current_node.push(AstNode::IncrementValue),
                Token::DecrementValue => current_node.push(AstNode::DecrementValue),
                Token::IncrementPointer => current_node.push(AstNode::IncrementPointer),
                Token::DecrementPointer => current_node.push(AstNode::DecrementPointer),
                Token::LoopBegin => {
                    stack.push(current_node);
                    current_node = Vec::new();
                }
                Token::LoopEnd => {
                    let loop_node = current_node;

                    current_node = stack
                        .pop()
                        .ok_or_else(|| anyhow::anyhow!("Unmatched ']'"))?;

                    current_node.push(AstNode::Loop(Ast { nodes: loop_node }));
                }
                Token::Input => current_node.push(AstNode::Input),
                Token::Output => current_node.push(AstNode::Output),
            }
        }

        Ok(Ast {
            nodes: current_node,
        })
    }
}
