use crate::ast;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_let_statements() {
    let input = String::from("let x = 5;
let y = 10;
let foobar = 838383;
");

    let mut lexer  = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);

    let program: Result<ast::Program<ast::Statement>, _> = parser.parse_program();
}
