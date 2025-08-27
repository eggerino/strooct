use crate::parsing::{
    ast::{Block, Statement},
    lexer::Lexer,
    token::{MarkedToken, SourceMarker, Token},
};

pub struct ParsingResult {
    pub blocks: Vec<Block>,
    pub errors: Vec<String>,
}

pub fn parse(lexer: Lexer) -> ParsingResult {
    let mut parser = Parser::create(lexer.filter(|x| !matches!(x.token, Token::Comment { .. })));
    parser.parse();
    return parser.into_result();
}

struct Parser<'a, S>
where
    S: Iterator<Item = MarkedToken<'a>>,
{
    stream: S,
    blocks: Vec<Block>,
    errors: Vec<String>,
    cur_tok: Option<MarkedToken<'a>>,
    peek_tok: Option<MarkedToken<'a>>,
}

impl<'a, S> Parser<'a, S>
where
    S: Iterator<Item = MarkedToken<'a>>,
{
    fn create(l: S) -> Self {
        let mut l = Self {
            stream: l,
            blocks: Vec::new(),
            errors: Vec::new(),
            cur_tok: None,
            peek_tok: None,
        };

        l.advance_token();
        l.advance_token();

        return l;
    }

    fn into_result(self) -> ParsingResult {
        ParsingResult {
            blocks: self.blocks,
            errors: self.errors,
        }
    }

    fn advance_token(&mut self) {
        self.cur_tok = self.peek_tok.take();
        self.peek_tok = self.stream.next();
    }

    fn parse(&mut self) {
        while self.cur_tok.is_some() {            
            if let Some(b) = self.parse_block() {
                self.blocks.push(b);
            }

            self.advance_token();
        }
    }

    fn parse_block(&mut self) -> Option<Block> {
        match &self.cur_tok {
            Some(cur) => match cur.token {
                Token::Program => self.parse_program(),
                Token::Action => todo!(),
                Token::Function => todo!(),
                Token::FunctionBlock => todo!(),
                Token::Var => todo!(),
                Token::Type => todo!(),
                _ => {
                    self.errors
                        .push(token_error(cur, "Not a block opening token."));
                    None
                }
            },
            None => {
                self.errors.push(
                    "Expected a block opening token. But no more tokens in the lexer.".to_owned(),
                );
                None
            }
        }
    }

    fn parse_program(&mut self) -> Option<Block> {
        self.advance_token();

        let identifier = match &self.cur_tok {
            Some(x) => match x.token {
                Token::Identifier(i) => Some(i.to_string()),
                _ => {
                    self.errors
                        .push(token_error(&x, "Expected an identifier token."));
                    None
                }
            },
            None => {
                self.errors
                    .push("Expected an identifier after the PROGRAM declaration.".to_string());
                None
            }
        }?;
        self.advance_token();

        let mut statements = Vec::new();
        while let Some(t) = &self.cur_tok
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

fn token_error(t: &MarkedToken, msg: &str) -> String {
    let err_line = get_marked_line(&t.marker);

    format!(
        "Error in {file} at line {line} column {col}\n\n{src}\n\n{arrow:>padding$}\n{arrow_body:>padding$}\n\n{msg}",
        file = t.marker.src_file,
        line = t.marker.line,
        col = t.marker.col,
        src = err_line,
        arrow = "^",
        padding = t.marker.col + 1,
        arrow_body = "|"
    )
}

fn get_marked_line<'a>(m: &'a SourceMarker) -> &'a str {
    let mut start_idx = 0;
    let mut src = m.src;

    for idx in 0..m.pos {
        if src.chars().next() == Some('\n') {
            start_idx = idx + 1;
        }
        src = &src[1..];
    }

    for end_idx in m.pos..m.src.len() {
        if src.chars().next() == Some('\n') {
            return &m.src[start_idx..end_idx];
        }

        src = &src[1..];
    }

    &m.src[start_idx..]
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

        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.blocks.len(), 1);

        assert_eq!(result.blocks[0], Block::Program("MyProgram".to_string(), vec![]));
    }
}
