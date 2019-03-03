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
    statements: Vec<Box<S>>
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




struct LetStatement<E: Expression> {
    name    : Identifier,
    value   : E,
}

impl<E> Statement for LetStatement<E>
    where E: Expression
{
    fn statement_node() {}
}

impl<E> Node for LetStatement<E>
    where E: Expression
{
    fn token_literal(&self) -> Option<String> {
        Some(String::from("let"))
    }
}




struct Identifier {
    value : String,
}

impl Expression for Identifier {
    fn expression_node() {}
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        Some(self.value.clone())
    }
}
