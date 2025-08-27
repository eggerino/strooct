use crate::parsing::{
    ast::{Ast, Block, Statement},
    lexer::Lexer,
    token::{MarkedToken, Token},
};

type Error = String;
type Errors = Vec<Error>;
type Result = std::result::Result<Ast, Errors>;

pub fn parse(lexer: Lexer) -> Result {
    let mut parser = Parser::create(lexer.filter(|x| !matches!(x.token, Token::Comment { .. })));
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
    fn create(mut l: S) -> Self {
        let first_token = l.next();
        let second_token = l.next();

        Self {
            stream: l,
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

    fn parse(&mut self) {
        // Parse until the stream is exhausted
        while self.cur.is_some() {
            // Gather all top level blocks to for the ast
            if let Some(block) = self.parse_block() {
                self.ast.push(block);
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
                _ => token_error_out(&mut self.errors, cur, "Not a block opening token."),
            },
            None => error_out(
                &mut self.errors,
                "Expected a block opening token. But no more tokens in the lexer.",
            ),
        }
    }

    fn parse_program(&mut self) -> Option<Block> {
        self.advance();

        let identifier = match &self.cur {
            Some(x) => match x.token {
                Token::Identifier(i) => Some(i.to_string()),
                _ => token_error_out(&mut self.errors, x, "Expected an identifier token."),
            },
            None => error_out(
                &mut self.errors,
                "Expected an identifier after the PROGRAM declaration.",
            ),
        }?;
        self.advance();

        let mut statements = Vec::new();
        while let Some(t) = &self.cur
            && !matches!(t.token, Token::EndProgram)
        {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
        }

        return Some(Block::Program(identifier, statements));
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        None
    }
}

fn error_out<T>(errors: &mut Errors, msg: &str) -> Option<T> {
    errors.push(format!("[ERR]\n   -> {}", msg));
    None
}

fn token_error_out<T>(errors: &mut Errors, t: &MarkedToken, msg: &str) -> Option<T> {
    errors.push(format!(
        "[ERR] in {} {}",
        t.marker.format_as_printable(),
        msg
    ));
    None
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
    use super::*;

    #[test]
    fn test_program_block() {
        let src_file = "Some file.st";
        let src = "PROGRAM MyProgram END_PROGRAM\n";
        let l = Lexer::create(src_file, src);

        let result = parse(l);

        assert!(result.is_ok());
        let ast = result.unwrap();

        assert_eq!(ast[0], Block::Program("MyProgram".to_string(), vec![]));
    }
}
