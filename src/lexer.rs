#[derive(Debug)]
pub struct Location {
    start: usize,
    end: usize,
}

impl Location {
    pub fn new(start: usize) -> Self {
        Location {
            start,
            end: start + 1,
        }
    }

    pub fn start(&self, start: usize) -> Self {
        Location {
            start,
            end: self.end,
        }
    }

    pub fn with_size(&self, size: usize) -> Self {
        Location {
            start: self.start,
            end: self.start + size,
        }
    }
}

#[derive(Debug, strum_macros::EnumString)]
pub enum Keyword {
    #[strum(serialize = "let")]
    Let,
    #[strum(serialize = "const")]
    Const,
    #[strum(serialize = "fun")]
    Fun,
}

#[derive(Debug)]
pub enum Operator {
    Assignment,
}

#[derive(Debug)]
pub enum Token {
    Whitespace { loc: Location },
    Integer { loc: Location, value: i32 },
    Char { loc: Location, value: char },
    String { loc: Location, value: String },
    Keyword { loc: Location, kind: Keyword },
    Identifier { loc: Location, name: String },
    Operator { loc: Location, op: Operator },
    Paren { loc: Location, open: bool },
    Bracket { loc: Location, open: bool },
    Brace { loc: Location, open: bool },
}

pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code }
    }
    pub fn lex(&self) -> Vec<Token> {
        let chars = self.code.chars().collect::<Vec<char>>();
        let mut tokens: Vec<Token> = vec![];
        let mut idx = 0usize;

        while idx < chars.len() {
            let mut loc = Location::new(idx);
            let char = chars[idx];

            let token = match char {
                ' ' | '\t' | '\n' => Some(Token::Whitespace { loc }),
                '(' => Some(Token::Paren { open: true, loc }),
                ')' => Some(Token::Paren { open: false, loc }),
                '[' => Some(Token::Bracket { open: true, loc }),
                ']' => Some(Token::Bracket { open: false, loc }),
                '{' => Some(Token::Brace { open: true, loc }),
                '}' => Some(Token::Brace { open: false, loc }),
                '\'' | '"' => {
                    let mut letters: Vec<char> = vec![];
                    let quote = char;
                    let is_char = quote == '\'';

                    idx += 1;
                    while idx < chars.len() {
                        if chars[idx] == quote && chars[idx - 1] != '\\' {
                            break;
                        }

                        letters.push(chars[idx]);
                        idx += 1;
                    }

                    if chars[idx] != quote {
                        panic!("Incomplete char/string literal: The closing quote is missing")
                    }

                    let location = loc.with_size(letters.len() + 2);
                    Some(if is_char {
                        if letters.len() != 1 {
                            panic!("char must hold exactly one letter");
                        }

                        Token::Char {
                            loc: location,
                            value: letters[0],
                        }
                    } else {
                        Token::String {
                            loc: location,
                            value: letters.iter().collect(),
                        }
                    })
                }
                #[rustfmt::skip]
                _ => {
                    if char.is_ascii_digit() {
                        let mut reversed_number: i32 = 0;
                        let mut len: usize = 0;

                        while idx < chars.len() && chars[idx].is_ascii_digit() {
                            reversed_number += (chars[idx] as i32 - '0' as i32) * (10i32.pow(len as u32));

                            len += 1;
                            idx += 1;
                        }

                        idx -= 1;

                        let mut number = 0;
                        while reversed_number != 0 {
                            let digit = reversed_number % 10;
                            number = number * 10 + digit;
                            reversed_number /= 10;
                        }

                        Some(Token::Integer {
                            loc: loc.with_size(len),
                            value: number,
                        })
                    } else if char.is_alphabetic() || char == '_' {
                        let mut letters: Vec<char> = vec![];

                        while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '_')
                        {
                            letters.push(chars[idx]);
                            idx += 1;
                        }

                        idx -= 1;

                        let location = loc.with_size(letters.len());
                        let word: String = letters.iter().collect();
                        match Keyword::try_from(word.as_str()) {
                            Ok(keyword) => Some(Token::Keyword {
                                loc: location,
                                kind: keyword,
                            }),
                            Err(_) => Some(Token::Identifier {
                                loc: location,
                                name: word,
                            }),
                        }
                    } else {
                        None
                    }
                },
            };

            match token {
                Some(tok) => tokens.push(tok),
                None => panic!("Encountered unknown input."),
            }

            idx += 1;
        }

        tokens
    }
}
