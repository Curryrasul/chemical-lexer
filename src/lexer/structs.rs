#[derive(Debug)]
pub enum TokenType {
    Chem(String),
    OperatorArrow,
    OperatorPlus,
    Number(String),
    Error(String),
}

#[derive(Debug)]
pub struct Position {
    pub line: i32,
    pub row: i32,
}

#[derive(Debug)]
pub struct Token {
    pub tt: TokenType,
    pub pos: Position,
}
