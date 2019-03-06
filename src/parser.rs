use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lex: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lex: Lexer) -> Parser {
        let cur_t  = lex.next_token();
        let peek_t = lex.next_token();
        Parser {
            lex: lex,
            cur_token: cur_t,
            peek_token: peek_t,
        }
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, &'static str>
    {
        Err("fail parsing program")
    }
}
