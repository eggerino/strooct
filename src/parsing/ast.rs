use crate::parsing::token::NumberValue;

pub struct Ast {
    pub blocks: Vec<Block>,
}

impl Ast {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]
pub enum Block {
    Program(String, Statements),
}

type Statements = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Empty,
    Expression(Expression),
    Return,
    Exit,
    If(IfCondition),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(LiteralExpression),
    Infix(InfixExpression),
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpression {
    Number(NumberValue),
    True,
    False,
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

#[derive(Debug, PartialEq)]
pub struct IfCondition {
    pub branch : IfConditionalBranch,
    pub alt_branches: Vec<IfConditionalBranch>,
    pub fallback: Option<Statements>,
}

#[derive(Debug, PartialEq)]
pub struct IfConditionalBranch {
    pub condition: Expression,
    pub statements: Statements,
}
