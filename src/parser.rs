use std::{fmt::Debug, iter::Peekable};

use itertools::Itertools;

use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub enum Expr {
    Inc(usize),
    Dec(usize),
    IncPtr(usize),
    DecPtr(usize),
    StartLoop,
    EndLoop,
    Print,
    Read,
}

pub struct Parser {
    token_stream: Peekable<Tokenizer>,
}

impl Parser {
    pub fn new(token_stream: Tokenizer) -> Self {
        Self {
            token_stream: token_stream.peekable(),
        }
    }
}

impl Iterator for Parser {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.token_stream.next() {
            let count = self
                .token_stream
                .peeking_take_while(|t| t == &token)
                .count()
                + 1;
            let expr = match token {
                Token::Inc => Expr::Inc(count),
                Token::Dec => Expr::Dec(count),
                Token::IncPtr => Expr::IncPtr(count),
                Token::DecPtr => Expr::DecPtr(count),
                Token::StartLoop => Expr::StartLoop,
                Token::EndLoop => Expr::EndLoop,
                Token::Print => Expr::Print,
                Token::Read => Expr::Read,
                Token::Invalid => unreachable!("Invalid token"),
            };
            Some(expr)
        } else {
            None
        }
    }
}
