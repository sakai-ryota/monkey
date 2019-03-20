pub trait Node {
    fn token_literal(&self) -> Option<String>;
}
pub trait Statement  : Node {
    fn who(&self) -> String;
}
pub trait Expression : Node {}



pub enum ASTNode {
    Program(Program),
    LetStatement(LetStatement),
    Identifier(Identifier),
}


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

    pub fn get(&self, i: usize) -> &Box<Statement> {
        &self.statements[i]
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




pub struct LetStatement {
    name    : Identifier,
    value   : Box<Expression>,
}

impl LetStatement {
    pub fn get_ident(&self) -> Option<String> {
        self.name.token_literal()
    }
}

impl Statement for LetStatement {
    fn who(&self) -> String {
        String::from("let")
    }
}

impl Node for LetStatement
{
    fn token_literal(&self) -> Option<String> {
        Some(String::from("let"))
    }
}




pub struct Identifier {
    value : String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        Some(self.value.clone())
    }
}
