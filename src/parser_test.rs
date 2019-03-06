use crate::ast;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_let_statements() {
    let input = String::from("let x = 5;
let y = 10;
let foobar = 838383;
");

    let lexer  = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();
}
