use crate::position::Position;

#[derive(Debug, PartialEq)]
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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            "=" => Some(Self::Assign),
            "==" => Some(Self::Eq),
            "!=" => Some(Self::Ne),
            "<" => Some(Self::Lt),
            ">" => Some(Self::Gt),
            "<=" => Some(Self::Le),
            ">=" => Some(Self::Ge),
            "{" => Some(Self::Lcb),
            "}" => Some(Self::Rcb),
            "[" => Some(Self::Lsb),
            "]" => Some(Self::Rsb),
            "(" => Some(Self::Lrb),
            ")" => Some(Self::Rrb),
            _ => None,
        }
    }
}

mod symbol_tests {
    use crate::token::Symbol;

    #[test]
    fn as_str() {
        assert_eq!(Symbol::Add.as_str(), "+")
    }

    #[test]
    fn from_str() {
        assert_eq!(Symbol::from_str("a"), None);
        assert_eq!(Symbol::from_str("+"), Some(Symbol::Add))
    }
}

#[derive(Debug, PartialEq)]
enum ReservedKeyword {
    Let,
}

impl ReservedKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReservedKeyword::Let => "let",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "let" => Some(Self::Let),
            _ => None,
        }
    }
}

mod reserved_keyword_tests {
    use crate::token::ReservedKeyword;

    #[test]
    fn as_str() {
        assert_eq!(ReservedKeyword::Let.as_str(), "let")
    }

    #[test]
    fn from_str() {
        assert_eq!(ReservedKeyword::from_str("a"), None);
        assert_eq!(ReservedKeyword::from_str("let"), Some(ReservedKeyword::Let))
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    Int(i64),
    Float(f64),
    String(String),
    Symbol(Symbol),
    Ident(String),
}

impl TokenKind {
    pub fn is_reserved_keyword(&self) -> bool {
        if let TokenKind::Ident(s) = self {
            ReservedKeyword::from_str(s).is_some()
        } else {
            false
        }
    }
}

pub struct Token<'a> {
    pub kind: TokenKind,
    pub position: Position,
    pub next: Option<&'a Token<'a>>,
}
