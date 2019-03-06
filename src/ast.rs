pub trait Node {
    fn token_literal(&self) -> Option<String>;
}
pub trait Statement  : Node {}
pub trait Expression : Node {}

pub struct Program {
    statements: Vec<Box<Statement>>
}

impl Program {
    pub fn new() -> Program {
        let stmts : Vec<Box<Statement>> = Vec::new();
        Program { statements : stmts }
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }
}

impl Node for Program
{
    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            None
        }
    }
}




struct LetStatement {
    name    : Identifier,
    value   : Box<Expression>,
}

impl Statement for LetStatement {}

impl Node for LetStatement
{
    fn token_literal(&self) -> Option<String> {
        Some(String::from("let"))
    }
}




struct Identifier {
    value : String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        Some(self.value.clone())
    }
}
