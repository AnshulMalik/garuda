use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenLocation {
    pub start: Location,
    pub end: Location
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    // reserved words
    Keyword(Keyword),
    String(String),
    Number(f64),
    Identifier(String),
    Symbol(Symbol)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    OpeningParen,
    ClosingParen,
    OpeningBoxBracket,
    ClosingBoxBracket,
    OpeningBrace,
    ClosingBrace,
    Dot,
    Spread,
    SemiColon,
    Comma,
    Lt,
    Gt,
    Le,
    Ge,
    Assign,
    Eq,
    SEq,
    Ne,
    SNe,
    Arrow,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Shl,
    Shr,
    ZFShr,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    Xor,
    BitwiseNot,
    Not,
    Question,
    Colon,
    AssignShl,
    AssignShr,
    AssignZFShr,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignMod,
    AssignPow,
    AssignAnd,
    AssignOr,
    AssignXor,
    Nothing
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Keyword::Var => "var",
            _ => "Nothing"
        };
        write!(f, "{}", printable)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub location: TokenLocation,
    pub kind: Kind
}

impl Token {
    pub fn new_keyword(location: TokenLocation, keyword: Kind) -> Token {
        Token {
            location,
            kind: keyword
        }
    }

    pub fn new_identifier(location: TokenLocation, name: String) -> Token {
        Token {
            location,
            kind: Kind::Identifier(name)
        }
    }
}

pub fn to_keyword(text: &str) -> Option<Keyword> {
    match text {
        "await" => Some(Keyword::Await),
        "break" => Some(Keyword::Break),
        "case" => Some(Keyword::Case),
        "catch" => Some(Keyword::Catch),
        "class" => Some(Keyword::Class),
        "const" => Some(Keyword::Const),
        "continue" => Some(Keyword::Continue),
        "debugger" => Some(Keyword::Debugger),
        "default" => Some(Keyword::Default),
        "delete" => Some(Keyword::Delete),
        "do" => Some(Keyword::Do),
        "else" => Some(Keyword::Else),
        "export" => Some(Keyword::Export),
        "extends" => Some(Keyword::Extends),
        "finally" => Some(Keyword::Finally),
        "for" => Some(Keyword::For),
        "function" => Some(Keyword::Function),
        "if" => Some(Keyword::If),
        "import" => Some(Keyword::Import),
        "in" => Some(Keyword::In),
        "instanceof" => Some(Keyword::Instanceof),
        "new" => Some(Keyword::New),
        "return" => Some(Keyword::Return),
        "super" => Some(Keyword::Super),
        "switch" => Some(Keyword::Switch),
        "this" => Some(Keyword::This),
        "throw" => Some(Keyword::Throw),
        "try" => Some(Keyword::Try),
        "typeof" => Some(Keyword::Typeof),
        "var" => Some(Keyword::Var),
        "void" => Some(Keyword::Void),
        "while" => Some(Keyword::While),
        "with" => Some(Keyword::With),
        "yield" => Some(Keyword::Yield),
        _ => None
    }
}
