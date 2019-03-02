use crate::token::{Token, KeywordsMap};

pub struct Lexer {
    src:           Vec<char>,
    position:      usize,
    read_position: usize,
    ch:            char,
    kwmap:         KeywordsMap,
}

impl Lexer {
    pub fn new(input: std::str::Chars) -> Lexer {
        let source: Vec<char>   = input.collect();
        let ch : char           = source[0];
        let kwmap               = KeywordsMap::new();
        let mut lex = Lexer {
            src             : source,
            position        : 0usize,
            read_position   : 0usize,
            ch              : ch,
            kwmap           : kwmap
        };
        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        if self.read_position >= self.src.len() {
            self.ch = '\0';
        } else {
            self.ch = self.src[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.src.len() {
            '\0'
        } else {
            self.src[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '='         => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            },
            '+'         => Token::PLUS,
            '-'         => Token::MINUS,
            '!'         => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NOTEQ
                } else {
                    Token::BANG
                }
            },
            '/'         => Token::SLASH,
            '*'         => Token::ASTERISK,
            '<'         => Token::LT,
            '>'         => Token::GT,
            ';'         => Token::SEMICOLON,
            '('         => Token::LPAREN,
            ')'         => Token::RPAREN,
            ','         => Token::COMMA,
            '{'         => Token::LBRACE,
            '}'         => Token::RBRACE,
            '\0'        => Token::EOF,
            '1' ... '9' => return self.read_number(),
            _           => {
                if Lexer::is_monkey_alphabetic(self.ch) {
                    return self.read_identiier()
                } else {
                    Token::ILLEGAL
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_number(&mut self) -> Token {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let literal: String = self.src[pos..self.position].iter().collect();
        Token::INT(literal)
    }

    fn read_identiier(&mut self) -> Token {
        let pos = self.position;
        while Lexer::is_monkey_alphabetic(self.ch) {
            self.read_char();
        }
        let literal: String = self.src[pos..self.position].iter().collect();
        self.kwmap.lookup_ident(literal)
    }

    fn is_monkey_alphabetic(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}
