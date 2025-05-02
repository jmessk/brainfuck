use std::io::Read as _;

use crate::parser::{Ast, AstNode};

pub struct Context {
    memory: Vec<u8>,
    pointer: usize,
}

impl Context {
    const DEFAULT_MEM_SIZE: usize = 10_000;
}

impl Default for Context {
    fn default() -> Self {
        Context::with_mem_size(Self::DEFAULT_MEM_SIZE)
    }
}

impl Context {
    pub fn with_mem_size(mem_size: usize) -> Self {
        Context {
            memory: vec![0; mem_size],
            pointer: 0,
        }
    }

    pub fn eval(&mut self, ast: &Ast) -> anyhow::Result<()> {
        for node in &ast.nodes {
            match node {
                AstNode::IncrementValue => self.increment_value()?,
                AstNode::DecrementValue => self.decrement_value()?,
                AstNode::IncrementPointer => self.increment_pointer()?,
                AstNode::DecrementPointer => self.decrement_pointer()?,
                AstNode::Loop(ast) => self.eval_loop(ast)?,
                AstNode::Input => self.input()?,
                AstNode::Output => self.output()?,
            }
        }

        Ok(())
    }

    fn increment_value(&mut self) -> anyhow::Result<()> {
        // the pointer cannot be out of bounds
        self.memory[self.pointer] = self.memory[self.pointer]
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("Value overflow"))?;

        Ok(())
    }

    fn decrement_value(&mut self) -> anyhow::Result<()> {
        // the pointer cannot be out of bounds
        self.memory[self.pointer] = self.memory[self.pointer]
            .checked_sub(1)
            .ok_or_else(|| anyhow::anyhow!("Value underflow"))?;

        Ok(())
    }

    fn increment_pointer(&mut self) -> anyhow::Result<()> {
        self.pointer = self
            .pointer
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("Pointer out of bounds"))?;

        Ok(())
    }

    fn decrement_pointer(&mut self) -> anyhow::Result<()> {
        self.pointer = self
            .pointer
            .checked_sub(1)
            .ok_or_else(|| anyhow::anyhow!("Pointer out of bounds"))?;

        Ok(())
    }

    fn eval_loop(&mut self, ast: &Ast) -> anyhow::Result<()> {
        while self.memory[self.pointer] != 0 {
            self.eval(ast)?;
        }

        Ok(())
    }

    fn input(&mut self) -> anyhow::Result<()> {
        let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|b| b.ok())
            .ok_or_else(|| anyhow::anyhow!("Failed to read input"))?;

        self.memory[self.pointer] = input;

        Ok(())
    }

    fn output(&mut self) -> anyhow::Result<()> {
        print!("{}", self.memory[self.pointer] as char);
        Ok(())
    }
}
