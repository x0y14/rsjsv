use crate::position::Position;

pub enum Symbol {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    Assign, // =

    Eq, // ==
    Ne, // !=
    Lt, // <
    Gt, // >
    Le, // <=
    Ge, // >=

    Lcb, // {
    Rcb, // }
    Lsb, // [
    Rsb, // ]
    Lrb, // (
    Rrb, // )
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::Add => "+",
            Symbol::Sub => "-",
            Symbol::Mul => "*",
            Symbol::Div => "/",
            Symbol::Assign => "=",
            Symbol::Eq => "==",
            Symbol::Ne => "!=",
            Symbol::Lt => "<",
            Symbol::Gt => ">",
            Symbol::Le => "<=",
            Symbol::Ge => ">=",
            Symbol::Lcb => "{",
            Symbol::Rcb => "}",
            Symbol::Lsb => "[",
            Symbol::Rsb => "]",
            Symbol::Lrb => "(",
            Symbol::Rrb => ")",
        }
    }
}

pub enum TokenKind {
    Eof,
    Int(i64),
    Float(f64),
    String(String),
    Symbol(Symbol),
    Ident(String),
}

pub struct Token<'a> {
    pub position: Position,
    pub next: Option<&'a Token<'a>>,
}
