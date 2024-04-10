use crate::token::Token;

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,

    current: usize,
    start: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source
                .chars()
                .collect(),
            tokens: Vec::new(),

            current: 0,
            start: 0,
            line: 1
        }
    }
}