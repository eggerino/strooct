use crate::parsing::{
    ast::{Ast, Block, Expression, IfCondition, IfConditionalBranch, LiteralExpression, Statement},
    lexer::Lexer,
    token::{MarkedToken, Token},
};

type Error = String;
type Errors = Vec<Error>;
type Result = std::result::Result<Ast, Errors>;

pub fn parse(lexer: Lexer) -> Result {
    let stream = lexer.filter(|x| !matches!(x.token, Token::Comment { .. }));
    let mut parser = Parser::create(stream);
    parser.parse();
    return parser.into_result();
}

struct Parser<'a, S>
where
    S: Iterator<Item = MarkedToken<'a>>,
{
    stream: S,
    ast: Ast,
    errors: Errors,
    cur: Option<MarkedToken<'a>>,
    peek: Option<MarkedToken<'a>>,
}

impl<'a, S> Parser<'a, S>
where
    S: Iterator<Item = MarkedToken<'a>>,
{
    fn create(mut stream: S) -> Self {
        let first_token = stream.next();
        let second_token = stream.next();

        Self {
            stream,
            ast: Ast::new(),
            errors: Errors::new(),
            cur: first_token,
            peek: second_token,
        }
    }

    fn into_result(self) -> Result {
        match self.errors[..] {
            [] => Ok(self.ast),
            _ => Err(self.errors),
        }
    }

    fn error_out<T>(&mut self, msg: &str) -> Option<T> {
        if let Some(cur) = &self.cur {
            self.errors.push(format!(
                "[ERR] in {} {}",
                cur.marker.format_as_printable(),
                msg
            ));
        } else {
            self.errors.push(format!("[ERR] {}", msg));
        }

        None
    }

    fn parse(&mut self) {
        // Parse until the stream is exhausted
        while self.cur.is_some() {
            // Gather all top level blocks to for the ast
            if let Some(block) = self.parse_block() {
                self.ast.blocks.push(block);
            }

            self.advance();
        }
    }

    fn advance(&mut self) {
        self.cur = self.peek.take();
        self.peek = self.stream.next();
    }

    fn parse_block(&mut self) -> Option<Block> {
        match &self.cur {
            Some(cur) => match cur.token {
                Token::Program => self.parse_program(),
                Token::Action => todo!(),
                Token::Function => todo!(),
                Token::FunctionBlock => todo!(),
                Token::Var => todo!(),
                Token::Type => todo!(),
                _ => self.error_out("Expected a block opening token."),
            },
            None => self.error_out("Expected a block opening token."),
        }
    }

    fn parse_program(&mut self) -> Option<Block> {
        self.advance();

        let identifier = match &self.cur {
            Some(x) => match x.token {
                Token::Identifier(i) => Some(i.to_string()),
                _ => self.error_out("Expected an identifier token after the PROGRAM declaration."),
            },
            None => self.error_out("Expected an identifier token after the PROGRAM declaration."),
        }?;
        self.advance();

        let mut statements = Vec::new();
        while let Some(cur) = &self.cur {
            // Correct ending of a program
            if cur.token == Token::EndProgram {
                return Some(Block::Program(identifier, statements));
            }

            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.advance();
        }

        self.error_out(&format!(
            "Program {identifier} is not properly closed. Try adding a END_PROGRAM to the end."
        ))
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.cur {
            Some(cur) => match cur.token {
                Token::SemiColon => Some(Statement::Empty),
                Token::Exit => {
                    self.advance();
                    Some(Statement::Exit)
                }
                Token::Return => {
                    self.advance();
                    Some(Statement::Return)
                }
                Token::If => self.parse_if_condition(),
                Token::Case => todo!(),
                Token::For => todo!(),
                Token::While => todo!(),
                Token::Pragma(_) => self.error_out("Pragmas are currently not supported"),
                _ => self.parse_expression_statement(),
            },
            None => self.error_out("Expected a statement but no more tokens left."),
        }
    }

    fn parse_if_condition(&mut self) -> Option<Statement> {
        self.advance();

        let mut condition = IfCondition {
            branch: IfConditionalBranch {
                condition: Expression::Literal(LiteralExpression::True),
                statements: Vec::new(),
            },
            alt_branches: Vec::new(),
            fallback: None,
        };

        if let Some(c) = self.parse_expression() {
            condition.branch.condition = c;
            self.advance();
        } else {
            return self.error_out("Expected an expression as a condition of an if statement.");
        }

        if let Some(cur) = &self.cur
            && cur.token == Token::Then
        {
            self.advance();
        } else {
            return self.error_out(
                "Expected the THEN keyword to separate the condition and the conditional code.",
            );
        }

        let mut statements = &mut condition.branch.statements;

        while let Some(cur) = &self.cur {
            if cur.token == Token::EndIf {
                self.advance();
                return Some(Statement::If(condition));
            }

            if cur.token == Token::Else {
                condition.fallback = Some(Vec::new());
                statements = condition.fallback.as_mut().unwrap();
            } else if cur.token == Token::Elsif {
                self.advance();
                if let Some(c) = self.parse_expression() {
                    condition.alt_branches.push(IfConditionalBranch {
                        condition: c,
                        statements: Vec::new(),
                    });
                    statements = &mut condition.alt_branches.last_mut().unwrap().statements;
                    self.advance();
                } else {
                    return self
                        .error_out("Expected an expression as a condition of an else if branch.");
                }

                if let Some(cur) = &self.cur
                    && cur.token == Token::Then
                {
                } else {
                    return self.error_out("Expected the THEN keyword to separate the condition and the conditional code.");
                }
            } else if let Some(s) = self.parse_statement() {
                statements.push(s);
            } else {
                self.error_out::<u8>("Expected statement inside code blocks of an if statement.");
            }

            self.advance();
        }

        None
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression()?;
        self.advance();

        match &self.cur {
            Some(cur) => match cur.token {
                Token::SemiColon => Some(Statement::Expression(expr)),
                _ => self.error_out("Expected a semi colon at the end of the statement."),
            },
            None => self.error_out("Statement at the end of the file is missing a semi colon."),
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match &self.cur {
            Some(cur) => match &cur.token {
                Token::Identifier(_) => todo!(),
                Token::Number(_, x) => {
                    Some(Expression::Literal(LiteralExpression::Number(x.clone())))
                }
                Token::String(_) => todo!(),
                Token::Time(_, time_value) => todo!(),
                Token::Plus => todo!(),
                Token::Minus => todo!(),
                Token::LeftParenthesis => todo!(),
                Token::Not => todo!(),
                Token::True => Some(Expression::Literal(LiteralExpression::True)),
                Token::False => Some(Expression::Literal(LiteralExpression::False)),
                _ => self.error_out("Invalid token kind for an expression."),
            },
            None => self.error_out("No more tokens left but expected an expression."),
        }
    }
}

const LOWEST_PRECEDENCE: u8 = 1;
const EQUALS_PRECEDENCE: u8 = 2;
const LESS_GREATER_PRECEDENCE: u8 = 3;
const SUM_PRECEDENCE: u8 = 4;
const PRODUCT_PRECEDENCE: u8 = 5;
const PREFIX_PRECEDENCE: u8 = 6;
const CALL_PRECEDENCE: u8 = 7;
const INDEX_PRECEDENCE: u8 = 8;

fn get_precedence(t: &Token) -> u8 {
    match t {
        Token::Plus => SUM_PRECEDENCE,
        Token::Minus => SUM_PRECEDENCE,
        Token::Asterisk => PRODUCT_PRECEDENCE,
        Token::Slash => PRODUCT_PRECEDENCE,
        Token::Percent => PRODUCT_PRECEDENCE,
        Token::Equals => EQUALS_PRECEDENCE,
        Token::NotEquals => EQUALS_PRECEDENCE,
        Token::GreaterThan => LESS_GREATER_PRECEDENCE,
        Token::GreaterThanOrEquals => LESS_GREATER_PRECEDENCE,
        Token::LessThan => LESS_GREATER_PRECEDENCE,
        Token::LessThanOrEquals => LESS_GREATER_PRECEDENCE,
        Token::Dot => INDEX_PRECEDENCE,
        Token::LeftParenthesis => CALL_PRECEDENCE,
        Token::LeftBracket => INDEX_PRECEDENCE,
        Token::Not => PREFIX_PRECEDENCE,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::{
        ast::{IfCondition, IfConditionalBranch},
        token::NumberValue,
    };

    use super::*;

    fn parse_src(src: &str) -> Ast {
        parse(Lexer::create("main.st", src)).unwrap()
    }

    #[test]
    fn test_empty_program_block() {
        let ast = parse_src("PROGRAM MyProgram END_PROGRAM\n");

        assert_eq!(
            ast.blocks[0],
            Block::Program("MyProgram".to_string(), vec![])
        );
    }

    #[test]
    fn test_statements() {
        let ast = parse_src(
            r#"
        PROGRAM MyProgram
            ;
            1;
            EXIT;
            RETURN;
        END_PROGRAM
        "#,
        );

        assert_eq!(
            ast.blocks[0],
            Block::Program(
                "MyProgram".to_string(),
                vec![
                    Statement::Empty,
                    Statement::Expression(Expression::Literal(LiteralExpression::Number(
                        NumberValue::Int(1)
                    ))),
                    Statement::Exit,
                    Statement::Return
                ]
            )
        );
    }

    #[test]
    fn test_if_conditions() {
        let ast = parse_src(
            r#"
        PROGRAM MyProgram
            IF TRUE THEN
                ;
            END_IF;
            IF FALSE THEN
                1;
            ELSIF TRUE THEN
                2;
            ELSIF FALSE THEN
                3;
            ELSE
                4;
            END_IF;
        END_PROGRAM
        "#,
        );

        assert_eq!(
            ast.blocks[0],
            Block::Program(
                "MyProgram".to_string(),
                vec![
                    Statement::If(IfCondition {
                        branch: IfConditionalBranch {
                            condition: Expression::Literal(LiteralExpression::True),
                            statements: vec![Statement::Empty],
                        },
                        alt_branches: vec![],
                        fallback: None,
                    }),
                    Statement::If(IfCondition {
                        branch: IfConditionalBranch {
                            condition: Expression::Literal(LiteralExpression::False),
                            statements: vec![Statement::Expression(Expression::Literal(
                                LiteralExpression::Number(NumberValue::Int(1))
                            ))]
                        },
                        alt_branches: vec![
                            IfConditionalBranch {
                                condition: Expression::Literal(LiteralExpression::True),
                                statements: vec![Statement::Expression(Expression::Literal(
                                    LiteralExpression::Number(NumberValue::Int(2))
                                ))]
                            },
                            IfConditionalBranch {
                                condition: Expression::Literal(LiteralExpression::False),
                                statements: vec![Statement::Expression(Expression::Literal(
                                    LiteralExpression::Number(NumberValue::Int(3))
                                ))]
                            },
                        ],
                        fallback: Some(vec![Statement::Expression(Expression::Literal(
                            LiteralExpression::Number(NumberValue::Int(4))
                        ))]),
                    })
                ]
            )
        );
    }
}
