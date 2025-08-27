use crate::parsing::token::NumberData;

#[derive(Debug, PartialEq)]
pub enum Block {
    Program(String, Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExpressionStatement(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    LiteralNumber(NumberData),
    Infix(InfixExpression),
}

#[derive(Debug, PartialEq)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: InfixOperator,
}

#[derive(Debug, PartialEq)]
pub enum InfixOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}
