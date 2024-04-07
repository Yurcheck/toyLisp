
pub struct Token {
    pub token_type: TokenType,
    pub line: usize
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Self {
            token_type,
            line
        }
    }
}

pub enum TokenType {
    LeftParen,
    RightParen,

    Operator(Operator),
    Literal(Literal),

    EOF
}

pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Not,
}

pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Identifier(String),
    Nil,
}