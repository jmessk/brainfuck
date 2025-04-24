use crate::parser::{Ast, AstNode};

pub struct Context {
    memory: Vec<u8>,
    pointer: usize,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            memory: vec![0; 1024],
            pointer: 0,
        }
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
        // println!("{}\n{}", self.pointer, self.memory[self.pointer]);
        for node in &ast.nodes {
            match node {
                AstNode::IncrementValue => self.increment_value()?,
                AstNode::DecrementValue => self.decrement_value()?,
                AstNode::IncrementPointer => self.increment_pointer()?,
                AstNode::DecrementPointer => self.decrement_pointer()?,
                AstNode::Loop(loop_nodes) => self.eval_loop(loop_nodes)?,
                AstNode::Input => self.input()?,
                AstNode::Output => self.output()?,
            }
        }

        Ok(())
    }

    fn increment_value(&mut self) -> anyhow::Result<()> {
        self.memory[self.pointer] += 1;
        Ok(())
    }

    fn decrement_value(&mut self) -> anyhow::Result<()> {
        self.memory[self.pointer] -= 1;
        Ok(())
    }

    fn increment_pointer(&mut self) -> anyhow::Result<()> {
        if self.pointer + 1 < self.memory.len() {
            self.pointer += 1;
        } else {
            anyhow::bail!("Pointer out of bounds");
        }

        Ok(())
    }

    fn decrement_pointer(&mut self) -> anyhow::Result<()> {
        if 0 < self.pointer {
            self.pointer -= 1;
        } else {
            anyhow::bail!("Pointer out of bounds");
        }

        Ok(())
    }

    fn eval_loop(&mut self, loop_nodes: &Ast) -> anyhow::Result<()> {
        self.eval(loop_nodes)?;

        while self.memory[self.pointer] != 0 {
            self.eval(loop_nodes)?;
        }

        Ok(())
    }

    fn input(&mut self) -> anyhow::Result<()> {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| anyhow::anyhow!("Failed to read input: {}", e))?;

        if let Ok(value) = input.trim().parse::<u8>() {
            self.memory[self.pointer] = value;
        } else {
            anyhow::bail!("Invalid input");
        }

        Ok(())
    }

    fn output(&mut self) -> anyhow::Result<()> {
        print!("{}", self.memory[self.pointer] as char);
        Ok(())
    }
}
