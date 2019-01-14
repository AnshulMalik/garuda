use token::{Token, Location, TokenLocation, Kind, Keyword, to_keyword, Symbol};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    EOF,
    Lexer(String)
}

#[derive(Clone, Debug)]
pub struct Lexer {
    source: String,
    pos: usize,
    line: usize,
    col: usize,
    buf: Vec<Token>
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            pos: 0,
            line: 1,
            col: 0,
            buf: vec![]
        }
    }

    pub fn print_tokens(&mut self) -> () {
        let mut i = 10;
        loop {
            match self.tokenize() {
                Ok(token) => {
                    println!("Token: {:?}", token);
                }
                Err(Error::EOF) => {
                    println!("End of line");
                    break;
                }
                _ => {
                    break;
                }
            }
            i -= 1;
            if i < 0 {
                break;
            }

        }
    }

    fn tokenize(&mut self) -> Result<Token, Error> {
        if self.source.starts_with("//") {
            // process single line comment
        } else if self.source.starts_with("/*") {
            // process multi line comment
        }

        return match self.read_next_char()? {
            'a'...'z' | 'A'...'Z' | '_' | '$' => self.read_identifier(),
            '0'...'9' => self.read_number("".to_string()),
            c if c.is_whitespace() => {
                self.read_whitespaces();
                self.tokenize()
            },
            // TODO: read_string
            // TODO: read template literal
            _ => self.read_symbol()
        }
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        // Identifier can be a keyword
        let mut name = "".to_string();
        let start = Location { line: self.line, col: self.col };

        loop {
            if let Ok(c) = self.read_next_char() {
                if c.is_alphanumeric() || c == '_' || c == '$' {
                    name.push(self.take_next_char()?);
                } else {
                    break;
                }
            } else  {
                break;
            }
        }

        let end = Location { line: self.line, col: self.col };

        match to_keyword(name.as_str()) {
            Some(keyword) => {

                let token = Token::new_keyword(TokenLocation { start, end }, Kind::Keyword(keyword));
                // self.buf.push(token);
                Ok(token)
            }
            None => {
                let token = Token::new_identifier(TokenLocation { start, end }, name);
                Ok(token)
            }
        }
    }

    fn skip_char_while<F>(&mut self, f: F) -> Result<(), Error>
    where F: Fn(char) -> bool
    {
        while !self.eof() && f(self.read_next_char()?) {
            self.take_next_char();
        }

        Ok(())
    }

    fn read_whitespaces(&mut self) -> Result<(), Error> {
        self.skip_char_while(|c| c.is_whitespace())
    }

    fn read_number(&mut self, mut s: String) -> Result<Token, Error> {
        let start = self.get_current_location();
        let mut dot_count = 0;
        loop {
            if let Ok(c) = self.read_next_char() {
                if c == '.' {
                    dot_count += 1;
                    if dot_count > 1 {
                        return Err(Error::Lexer("Failed to parse".to_string()));
                    }
                }
                if c.is_digit(10) || c == '.' {
                    s.push(self.take_next_char()?);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let end = self.get_current_location();
        let token = Token {
            location: TokenLocation { start, end },
            kind: Kind::Number(match s.parse() {
                Ok(n) => n,
                Err(_) => {
                    return Err(Error::Lexer("Failed to parse number".to_string()));
                }
            })
        };
        Ok(token)
    }
    fn get_current_location(&self) -> Location {
        Location { line: self.line, col: self.col }
    }

    fn read_symbol(&mut self) -> Result<Token, Error> {
        let start = Location { line: self.line, col: self.col };
        let mut symbol = Symbol::Nothing;
        match self.take_next_char()? {
            '=' => {
                symbol = Symbol::Assign;
                if self.take_next_char_if('=')? {
                    symbol = Symbol::Eq;
                    if self.take_next_char_if('=')? {
                        symbol = Symbol::SEq;
                    }
                } else if self.take_next_char_if('>')? {
                    symbol = Symbol::Arrow;
                }
            }
            '(' => symbol = Symbol::OpeningParen,
            ')' => symbol = Symbol::ClosingParen,
            '[' => symbol = Symbol::OpeningBoxBracket,
            ']' => symbol = Symbol::ClosingBoxBracket,
            '{' => symbol = Symbol::OpeningBrace,
            '}' => symbol = Symbol::ClosingBrace,
            '.' => {
                symbol = Symbol::Dot;
                if self.take_next_char_if('.')? {
                    if self.take_next_char_if('.')? {
                        symbol = Symbol::Spread;
                    } else {
                        // TODO: Raise error, `..` is no symbol
                    }
                } else if self.read_next_char()?.is_digit(10) {
                    return self.read_number(".".to_string());
                }
                // TODO Read Decimal number if .134 or .13e10
            },
            ';' => symbol = Symbol::SemiColon,
            ',' => symbol = Symbol::Comma,
            '?' => symbol = Symbol::Question,
            ':' => symbol = Symbol::Colon,
            '~' => symbol = Symbol::BitwiseNot,
            '+' => {
                symbol = Symbol::Add;
                if self.take_next_char_if('+')? {
                    symbol = Symbol::Inc;
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignAdd;
                }
            },
            '-' => {
                symbol = Symbol::Sub;
                if self.take_next_char_if('-')? {
                    symbol = Symbol::Dec;
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignSub;
                }
            },
            '!' => {
                symbol = Symbol::Not;
                if self.take_next_char_if('=')? {
                    symbol = Symbol::Ne;
                    if self.take_next_char_if('=')? {
                        symbol = Symbol::SNe;
                    }
                }
            },
            '%' => {
                symbol = Symbol::Mod;
                if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignMod;
                }
            },
            '^' => {
                symbol = Symbol::Xor;
                if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignXor;
                }
            }
            '|' => {
                symbol = Symbol::BitwiseOr;
                if self.take_next_char_if('|')? {
                    symbol = Symbol::Or;
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignOr;
                }
            },
            '&' => {
                symbol = Symbol::BitwiseAnd;
                if self.take_next_char_if('&')? {
                    symbol = Symbol::And;
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignAnd;
                }
            },
            '*' => {
                symbol = Symbol::Mul;
                if self.take_next_char_if('*')? {
                    symbol = Symbol::Pow;
                    if self.take_next_char_if('=')? {
                        symbol = Symbol::AssignPow;
                    }
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::AssignMul;
                }
            },
            '<' => {
                symbol = Symbol::Lt;
                if self.take_next_char_if('<')? {
                    symbol = Symbol::Shl;
                    if self.take_next_char_if('=')? {
                        symbol = Symbol::AssignShl;
                    }
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::Le;
                }
            },
            '>' => {
                symbol = Symbol::Gt;
                if self.take_next_char_if('>')? {
                    symbol = Symbol::Shr;
                    if self.take_next_char_if('>')? {
                        symbol = Symbol::ZFShr;
                        if self.take_next_char_if('=')? {
                            symbol = Symbol::AssignZFShr;
                        }
                    } else if self.take_next_char_if('=')? {
                        symbol = Symbol::AssignShr;
                    }
                } else if self.take_next_char_if('=')? {
                    symbol = Symbol::Ge;
                }
            }
            _ => {}
        };

        let end = Location { line: self.line, col: self.col };

        Ok(Token {
            location: TokenLocation { start, end },
            kind: Kind::Symbol(symbol)
        })
    }

    fn read_next_char(&self) -> Result<char, Error> {
        self.source[self.pos..].chars().next().ok_or(Error::EOF)
    }

    fn take_next_char_if(&mut self, c: char) -> Result<bool, Error> {
        if self.eof() {
            return Ok(false);
        }

        if self.read_next_char()? == c {
            self.take_next_char();
            return Ok(true);
        }
        Ok(false)
    }

    fn take_next_char(&mut self) -> Result<char, Error> {
        let res = self.read_next_char()?;
        self.pos += 1;
        self.col += 1;
        Ok(res)
    }

    fn eof(&self) -> bool {
        self.pos >= self.source.len()
    }
}

#[test]
fn symbols() {
    let mut lexer = Lexer::new("{ } [ ] = ==".to_string());
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Symbol(Symbol::OpeningBrace));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Symbol(Symbol::ClosingBrace));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Symbol(Symbol::OpeningBoxBracket));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Symbol(Symbol::ClosingBoxBracket));

    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Symbol(Symbol::Assign));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Symbol(Symbol::Eq));
}

#[test]
fn numbers() {
    let mut lexer = Lexer::new("123".to_string());
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Number(123 as f64));

}

#[test]
fn keywords() {
    let mut lexer = Lexer::new(
        "await break case catch class const continue debugger  \
         default delete do else export extends finally for \
         function if import in instanceof new return super \
         switch this throw try typeof var void while with yield"
            .to_string()
    );

    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Await));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Break));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Case));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Catch));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Class));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Const));

    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Continue));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Debugger));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Default));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Delete));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Do));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Else));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Export));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Extends));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Finally));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::For));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Function));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::If));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Import));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::In));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Instanceof));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::New));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Return));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Super));

    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Switch));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::This));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Throw));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Try));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Typeof));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Var));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Void));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::While));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::With));
    assert_eq!(lexer.tokenize().unwrap().kind, Kind::Keyword(Keyword::Yield));
}