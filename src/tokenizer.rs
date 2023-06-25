use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    StartLoop,
    EndLoop,
    Print,
    Read,
    Invalid,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '+' => Token::Inc,
            '-' => Token::Dec,
            '>' => Token::IncPtr,
            '<' => Token::DecPtr,
            '[' => Token::StartLoop,
            ']' => Token::EndLoop,
            '.' => Token::Print,
            ',' => Token::Read,
            _ => Token::Invalid,
        }
    }
}

pub struct Tokenizer {
    buffer: VecDeque<char>,
}

impl Tokenizer {
    pub fn new(path: &str) -> Result<Tokenizer> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(Tokenizer {
            buffer: contents.trim().chars().collect(),
        })
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.buffer.pop_front() {
            let token = Token::from(char);
            if token == Token::Invalid {
                self.next()
            } else {
                Some(token)
            }
        } else {
            None
        }
    }
}
