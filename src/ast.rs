pub trait Node {
    fn token_literal(&self) -> Option<String>;
}

pub trait Statement : Node {
    fn statement_node();
}

pub trait Expression : Node {
    fn expression_node();
}

pub struct Program<S: Statement> {
    statements: Vec<S>
}

impl<S> Node for Program<S>
    where S: Statement
{
    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            None
        }
    }
}
