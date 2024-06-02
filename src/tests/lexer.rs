#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use std::ops::Deref;

    #[test]
    fn parse_parens() {
        let lexer = Lexer::new("([{}])".into());
        let mut tokens = lexer.lex().into_iter();

        match tokens.next() {
            Some(Token::Paren { open: true, .. }) => { /* do nothing */ }
            _ => panic!("Expected '('"),
        }

        match tokens.next() {
            Some(Token::Bracket { open: true, .. }) => { /* do nothing */ }
            _ => panic!("Expected '['"),
        }

        match tokens.next() {
            Some(Token::Brace { open: true, .. }) => { /* do nothing */ }
            _ => panic!("Expected '{{'"),
        }

        match tokens.next() {
            Some(Token::Brace { open: false, .. }) => { /* do nothing */ }
            _ => panic!("Expected '}}'"),
        }

        match tokens.next() {
            Some(Token::Bracket { open: false, .. }) => { /* do nothing */ }
            _ => panic!("Expected ']'"),
        }

        match tokens.next() {
            Some(Token::Paren { open: false, .. }) => { /* do nothing */ }
            _ => panic!("Expected ')'"),
        }
    }

    #[test]
    fn parse_integer() {
        let lexer = Lexer::new("123".into());
        let tokens = lexer.lex();

        match tokens.first() {
            Some(Token::Integer { .. }) => { /* do nothing */ }
            _ => panic!("Expected an integer"),
        }
    }

    #[test]
    fn parse_identifier() {
        let lexer = Lexer::new("hi_there".into());
        let tokens = lexer.lex();

        match tokens.first() {
            Some(Token::Identifier { .. }) => { /* do nothing */ }
            _ => panic!("Expected an identifier"),
        }
    }

    #[test]
    #[rustfmt::skip]
    fn parse_keyword() {
        let lexer = Lexer::new("let const fun".into());
        let mut tokens = lexer.lex().into_iter().filter(|x| match x {
            Token::Whitespace { .. } => /* filter out whitespace */ false,
            _ => true
        });

        match tokens.next() {
            Some(Token::Keyword { kind: Keyword::Let, .. }) => { /* do nothing */ }
            _ => panic!("Expected the keyword 'let'"),
        }

        match tokens.next() {
            Some(Token::Keyword { kind: Keyword::Const, .. }) => { /* do nothing */ }
            _ => panic!("Expected the keyword 'const'"),
        }

        match tokens.next() {
            Some(Token::Keyword { kind: Keyword::Fun, .. }) => { /* do nothing */ }
            _ => panic!("Expected the keyword 'fun'"),
        }
    }

    #[test]
    fn parse_char() {
        let lexer = Lexer::new("'a'".into());
        let mut tokens = lexer.lex().into_iter();

        match tokens.next() {
            Some(Token::Char { value: 'a', .. }) => { /* do nothing */ }
            _ => panic!("Expected the character literal 'a'"),
        }
    }

    #[test]
    fn parse_string() {
        let lexer = Lexer::new("\"hi there\"".into());
        let mut tokens = lexer.lex();

        let mut error = false;
        if let Some(Token::String { value, .. }) = tokens.first() {
            if value != "hi there" {
                error = true;
            }
        } else {
            error = true;
        }

        if error {
            panic!("Expected the string literal \"hi there\"")
        }
    }
}
