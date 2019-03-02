use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOTEQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct KeywordsMap {
    map: HashMap<String, Token>
}

impl KeywordsMap {
    pub fn new() -> KeywordsMap {
        let map: HashMap<String, Token> = [(String::from("let"),Token::LET),
                                           (String::from("fn"), Token::FUNCTION),
                                           (String::from("true"), Token::TRUE),
                                           (String::from("false"), Token::FALSE),
                                           (String::from("if"), Token::IF),
                                           (String::from("else"), Token::ELSE),
                                           (String::from("return"), Token::RETURN)]
                                                .iter().cloned().collect();
        KeywordsMap { map : map }
    }

    pub fn lookup_ident(&self, ident: String) -> Token {
        match self.map.get(&ident) {
            Some(tok)   => tok.clone(),
            None        => Token::IDENT(ident),
        }
    }
}
