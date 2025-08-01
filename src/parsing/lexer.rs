use crate::parsing::token::{MarkedToken, NumberData, SourceMarker, TimeData, Token};
use nom::number::complete::double;

fn numeric_len(s: &str) -> Option<usize> {
    double::<_, ()>(s).map(|(r, _)| s.len() - r.len()).ok()
}

pub struct Lexer<'a> {
    src: &'a str,
    marker: SourceMarker<'a>,
}

type TokenResult<'a> = Option<(Token<'a>, usize)>;

impl<'a> Lexer<'a> {
    pub fn create(src_file: &'a str, src: &'a str) -> Self {
        Self {
            src,
            marker: SourceMarker::create_start(src_file),
        }
    }

    fn is_exhausted(&self) -> bool {
        self.src.len() == 0
    }

    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(c) = self.src.chars().next() {
                // Advance source
                self.src = &self.src[1..];

                // Advance marker with line breaks
                if c == '\n' {
                    self.marker.advance_new_line();
                } else {
                    self.marker.advance();
                }
            } else {
                // Stop when the end of the source is reached
                return;
            }
        }
    }

    fn cur_whitespace_len(&self) -> usize {
        self.src
            .chars()
            .take_while(|c| c.is_ascii_whitespace())
            .count()
    }

    fn get_token(&self) -> (Token<'a>, usize) {
        Token::from_keyword(self.src)
            .or_else(|| self.get_line_comment_token())
            .or_else(|| self.get_block_comment_token())
            .or_else(|| self.get_pragma_token())
            .or_else(|| self.get_time_token())
            .or_else(|| self.get_identifier_token())
            .or_else(|| self.get_string_token())
            .or_else(|| self.get_operator_token())
            .or_else(|| self.get_number_token())
            .or_else(|| self.get_delimiter_token())
            .unwrap_or((Token::Illegal, self.src.len()))
    }

    fn get_line_comment_token(&self) -> TokenResult<'a> {
        if !self.src.starts_with("//") {
            return None;
        }

        let n = self.src.chars().take_while(|c| *c != '\n').count();

        return Some((Token::Comment(&self.src[..n]), n));
    }

    fn get_block_comment_token(&self) -> TokenResult<'a> {
        if !self.src.starts_with("(*") {
            return None;
        }

        let mut peak = &self.src[2..];
        while peak.len() > 0 {
            if peak.starts_with("*)") {
                let n = self.src.len() - peak.len() + 2;
                return Some((Token::Comment(&self.src[..n]), n));
            }

            peak = &peak[1..];
        }
        return None;
    }

    fn get_pragma_token(&self) -> Option<(Token<'a>, usize)> {
        if !self.src.starts_with("#") {
            return None;
        }

        let n = self
            .src
            .chars()
            .take_while(|c| !c.is_ascii_whitespace())
            .count();

        return Some((Token::Pragma(&self.src[..n]), n));
    }

    fn get_time_token(&self) -> Option<(Token<'a>, usize)> {
        // Must start with "T#"
        if !self.src.starts_with("T#") {
            return None;
        }

        let mut data = TimeData {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milli_seconds: 0,
        };
        let mut cur_value: usize;
        let mut peak = &self.src[2..];
        if let Some(n) = numeric_len(peak)
            && let Ok(value) = str::parse::<usize>(&peak[..n])
        {
            cur_value = value;
            peak = &peak[n..];
        } else {
            return None;
        }

        for (lower, upper) in [("d", "D"), ("h", "H"), ("m", "M"), ("s", "S"), ("ms", "MS")] {
            if !peak.starts_with(lower) && !peak.starts_with(upper) {
                continue;
            }

            match lower {
                "d" => data.days = cur_value,
                "h" => data.hours = cur_value as u8,
                "m" => data.minutes = cur_value as u8,
                "s" => data.seconds = cur_value as u8,
                "ms" => data.milli_seconds = cur_value as u16,
                _ => unreachable!("Cannot pass check for d, h, m, s, ms and not be one of them"),
            }

            peak = &peak[lower.len()..];
            if let Some(n) = numeric_len(peak)
                && let Ok(value) = str::parse::<usize>(&peak[..n])
            {
                // Consume the number
                cur_value = value;
                peak = &peak[n..];
            } else {
                // Time is done when no number comes after an unit
                let len = self.src.len() - peak.len();
                return Some((Token::Time(&self.src[..len], data), len));
            }
        }

        // Invalid time when loop runs through
        return None;
    }

    fn get_identifier_token(&self) -> Option<(Token<'a>, usize)> {
        let first_char = self.src.chars().next()?;
        if first_char != '_' && !first_char.is_ascii_alphabetic() {
            return None;
        }

        let len = self
            .src
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .count();
        return Some((Token::Identifier(&self.src[..len]), len));
    }

    fn get_string_token(&self) -> Option<(Token<'a>, usize)> {
        let start_quote = self.src.chars().next()?;

        if start_quote != '"' && start_quote != '\'' {
            return None;
        }

        let len = self
            .src
            .chars()
            .skip(1)
            .take_while(|c| *c != start_quote)
            .count()
            + 2;

        if len <= self.src.len() {
            return Some((Token::String(&self.src[..len]), len));
        } else {
            return None;
        }
    }

    fn get_number_token(&self) -> Option<(Token<'a>, usize)> {
        match numeric_len(self.src) {
            None => None,
            Some(n) => match str::parse::<usize>(&self.src[..n]) {
                Ok(int) => Some((
                    Token::Number(&self.src[..n], NumberData::create_integer(int)),
                    n,
                )),
                _ => Some((
                    Token::Number(
                        &self.src[..n],
                        NumberData::create_float(
                            str::parse(&self.src[..n])
                                .expect("Must be a float when numeric and not integer"),
                        ),
                    ),
                    n,
                )),
            },
        }
    }

    fn get_operator_token(&self) -> Option<(Token<'a>, usize)> {
        match self.src {
            s if s.starts_with("+") => Some((Token::Plus, 1)),
            s if s.starts_with("-") => Some((Token::Minus, 1)),
            s if s.starts_with("*") => Some((Token::Asterisk, 1)),
            s if s.starts_with("/") => Some((Token::Slash, 1)),
            s if s.starts_with(":=") => Some((Token::Assign, 2)),
            s if s.starts_with("=") => Some((Token::Equals, 1)),
            s if s.starts_with("<>") => Some((Token::NotEquals, 2)),
            s if s.starts_with(">=") => Some((Token::GreaterThanOrEquals, 2)),
            s if s.starts_with(">") => Some((Token::GreaterThan, 1)),
            s if s.starts_with("<=") => Some((Token::LessThanOrEquals, 2)),
            s if s.starts_with("<") => Some((Token::LessThan, 1)),
            _ => None,
        }
    }

    fn get_delimiter_token(&self) -> Option<(Token<'a>, usize)> {
        let first_char = self.src.chars().next()?;

        let data = match first_char {
            '.' => Some(Token::Dot),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            ';' => Some(Token::SemiColon),
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            _ => None,
        }?;

        return Some((data, 1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = MarkedToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespaces (if any)
        self.advance(self.cur_whitespace_len());

        if self.is_exhausted() {
            return None;
        }

        let (token, token_len) = self.get_token();
        let marked_token = token.mark(self.marker.clone());
        self.advance(token_len);

        return Some(marked_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exp(
        t: Token<'static>,
        src_file: &'static str,
        pos: usize,
        line: usize,
        col: usize,
    ) -> Option<MarkedToken<'static>> {
        Some(t.mark(SourceMarker {
            src_file,
            pos,
            line,
            col,
        }))
    }

    #[test]
    fn test_whitespace() {
        let src_file = "Some file.st";
        let src = " \t\n\r\n  ";
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), None);
        assert_eq!(
            l.marker,
            SourceMarker {
                src_file,
                pos: 7,
                line: 2,
                col: 2
            }
        );
    }

    #[test]
    fn test_keywords() {
        let src_file = "Some file.st";
        let src = r#"NOT
TRUE
FALSE
AND
OR
XOR
IF
THEN
ELSIF
ELSE
END_IF
CASE
OF
END_CASE
FOR
TO
BY
DO
END_FOR
WHILE
END_WHILE
PROGRAM
END_PROGRAM
EXIT
ACTION
END_ACTION
FUNCTION
END_FUNCTION
FUNCTION_BLOCK
END_FUNCTION_BLOCK
RETURN
VAR
VAR_INPUT
VAR_OUTPUT
CONSTANT
END_VAR
TYPE
END_TYPE
STRUCT
END_STRUCT
UNION
END_UNION
"#;
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), exp(Token::Not, src_file, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::True, src_file, 4, 1, 0));
        assert_eq!(l.next(), exp(Token::False, src_file, 9, 2, 0));
        assert_eq!(l.next(), exp(Token::And, src_file, 15, 3, 0));
        assert_eq!(l.next(), exp(Token::Or, src_file, 19, 4, 0));
        assert_eq!(l.next(), exp(Token::Xor, src_file, 22, 5, 0));
        assert_eq!(l.next(), exp(Token::If, src_file, 26, 6, 0));
        assert_eq!(l.next(), exp(Token::Then, src_file, 29, 7, 0));
        assert_eq!(l.next(), exp(Token::Elsif, src_file, 34, 8, 0));
        assert_eq!(l.next(), exp(Token::Else, src_file, 40, 9, 0));
        assert_eq!(l.next(), exp(Token::EndIf, src_file, 45, 10, 0));
        assert_eq!(l.next(), exp(Token::Case, src_file, 52, 11, 0));
        assert_eq!(l.next(), exp(Token::Of, src_file, 57, 12, 0));
        assert_eq!(l.next(), exp(Token::EndCase, src_file, 60, 13, 0));
        assert_eq!(l.next(), exp(Token::For, src_file, 69, 14, 0));
        assert_eq!(l.next(), exp(Token::To, src_file, 73, 15, 0));
        assert_eq!(l.next(), exp(Token::By, src_file, 76, 16, 0));
        assert_eq!(l.next(), exp(Token::Do, src_file, 79, 17, 0));
        assert_eq!(l.next(), exp(Token::EndFor, src_file, 82, 18, 0));
        assert_eq!(l.next(), exp(Token::While, src_file, 90, 19, 0));
        assert_eq!(l.next(), exp(Token::EndWhile, src_file, 96, 20, 0));
        assert_eq!(l.next(), exp(Token::Program, src_file, 106, 21, 0));
        assert_eq!(l.next(), exp(Token::EndProgram, src_file, 114, 22, 0));
        assert_eq!(l.next(), exp(Token::Exit, src_file, 126, 23, 0));
        assert_eq!(l.next(), exp(Token::Action, src_file, 131, 24, 0));
        assert_eq!(l.next(), exp(Token::EndAction, src_file, 138, 25, 0));
        assert_eq!(l.next(), exp(Token::Function, src_file, 149, 26, 0));
        assert_eq!(l.next(), exp(Token::EndFunction, src_file, 158, 27, 0));
        assert_eq!(l.next(), exp(Token::FunctionBlock, src_file, 171, 28, 0));
        assert_eq!(l.next(), exp(Token::EndFunctionBlock, src_file, 186, 29, 0));
        assert_eq!(l.next(), exp(Token::Return, src_file, 205, 30, 0));
        assert_eq!(l.next(), exp(Token::Var, src_file, 212, 31, 0));
        assert_eq!(l.next(), exp(Token::VarInput, src_file, 216, 32, 0));
        assert_eq!(l.next(), exp(Token::VarOutput, src_file, 226, 33, 0));
        assert_eq!(l.next(), exp(Token::Constant, src_file, 237, 34, 0));
        assert_eq!(l.next(), exp(Token::EndVar, src_file, 246, 35, 0));
        assert_eq!(l.next(), exp(Token::Type, src_file, 254, 36, 0));
        assert_eq!(l.next(), exp(Token::EndType, src_file, 259, 37, 0));
        assert_eq!(l.next(), exp(Token::Struct, src_file, 268, 38, 0));
        assert_eq!(l.next(), exp(Token::EndStruct, src_file, 275, 39, 0));
        assert_eq!(l.next(), exp(Token::Union, src_file, 286, 40, 0));
        assert_eq!(l.next(), exp(Token::EndUnion, src_file, 292, 41, 0));

        assert_eq!(l.next(), None);
    }

    #[test]
    fn test_operators() {
        let src_file = "Some file.st";
        let src = r#"+
-
*
/
:=
=
<>
>
>=
<
<="#;
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), exp(Token::Plus, src_file, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::Minus, src_file, 2, 1, 0));
        assert_eq!(l.next(), exp(Token::Asterisk, src_file, 4, 2, 0));
        assert_eq!(l.next(), exp(Token::Slash, src_file, 6, 3, 0));
        assert_eq!(l.next(), exp(Token::Assign, src_file, 8, 4, 0));
        assert_eq!(l.next(), exp(Token::Equals, src_file, 11, 5, 0));
        assert_eq!(l.next(), exp(Token::NotEquals, src_file, 13, 6, 0));
        assert_eq!(l.next(), exp(Token::GreaterThan, src_file, 16, 7, 0));
        assert_eq!(
            l.next(),
            exp(Token::GreaterThanOrEquals, src_file, 18, 8, 0)
        );
        assert_eq!(l.next(), exp(Token::LessThan, src_file, 21, 9, 0));
        assert_eq!(l.next(), exp(Token::LessThanOrEquals, src_file, 23, 10, 0));

        assert_eq!(l.next(), None);
    }

    #[test]
    fn test_delimiters() {
        let src_file = "Some file.st";
        let src = r#".
,
:
;
(
)
[
]
{
}"#;
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), exp(Token::Dot, src_file, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::Comma, src_file, 2, 1, 0));
        assert_eq!(l.next(), exp(Token::Colon, src_file, 4, 2, 0));
        assert_eq!(l.next(), exp(Token::SemiColon, src_file, 6, 3, 0));
        assert_eq!(l.next(), exp(Token::LeftParenthesis, src_file, 8, 4, 0));
        assert_eq!(l.next(), exp(Token::RightParenthesis, src_file, 10, 5, 0));
        assert_eq!(l.next(), exp(Token::LeftBracket, src_file, 12, 6, 0));
        assert_eq!(l.next(), exp(Token::RightBracket, src_file, 14, 7, 0));
        assert_eq!(l.next(), exp(Token::LeftBrace, src_file, 16, 8, 0));
        assert_eq!(l.next(), exp(Token::RightBrace, src_file, 18, 9, 0));

        assert_eq!(l.next(), None);
    }

    #[test]
    fn test_literals() {
        let src_file = "Some file.st";
        let src = r#""Hello World'"
'Hello World"'
Identier_123
_private_Identifier321
_123Identifier
1
234
1.23
1.23e8
2.34E+3
4.21e-4
T#1s
T#1D1H1M1S1MS
T#1d1h1m1s1ms
T#1m1ms"#;
        let mut l = Lexer::create(src_file, src);

        assert_eq!(
            l.next(),
            exp(Token::String("\"Hello World'\""), src_file, 0, 0, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::String("'Hello World\"'"), src_file, 15, 1, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::Identifier("Identier_123"), src_file, 30, 2, 0)
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Identifier("_private_Identifier321"),
                src_file,
                43,
                3,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(Token::Identifier("_123Identifier"), src_file, 66, 4, 0)
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("1", NumberData::create_integer(1)),
                src_file,
                81,
                5,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("234", NumberData::create_integer(234)),
                src_file,
                83,
                6,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("1.23", NumberData::create_float(1.23)),
                src_file,
                87,
                7,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("1.23e8", NumberData::create_float(1.23e8)),
                src_file,
                92,
                8,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("2.34E+3", NumberData::create_float(2.34e3)),
                src_file,
                99,
                9,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("4.21e-4", NumberData::create_float(4.21e-4)),
                src_file,
                107,
                10,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Time(
                    "T#1s",
                    TimeData {
                        days: 0,
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milli_seconds: 0
                    }
                ),
                src_file,
                115,
                11,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Time(
                    "T#1D1H1M1S1MS",
                    TimeData {
                        days: 1,
                        hours: 1,
                        minutes: 1,
                        seconds: 1,
                        milli_seconds: 1
                    }
                ),
                src_file,
                120,
                12,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Time(
                    "T#1d1h1m1s1ms",
                    TimeData {
                        days: 1,
                        hours: 1,
                        minutes: 1,
                        seconds: 1,
                        milli_seconds: 1
                    }
                ),
                src_file,
                134,
                13,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Time(
                    "T#1m1ms",
                    TimeData {
                        days: 0,
                        hours: 0,
                        minutes: 1,
                        seconds: 0,
                        milli_seconds: 1
                    }
                ),
                src_file,
                148,
                14,
                0
            )
        );

        assert_eq!(l.next(), None);
    }

    #[test]
    fn test_pragma() {
        let src_file = "Some file.st";
        let src = "#include \"header.h\" #load \"variables.var\"";
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), exp(Token::Pragma("#include"), src_file, 0, 0, 0));
        assert_eq!(
            l.next(),
            exp(Token::String("\"header.h\""), src_file, 9, 0, 9)
        );
        assert_eq!(l.next(), exp(Token::Pragma("#load"), src_file, 20, 0, 20));
        assert_eq!(
            l.next(),
            exp(Token::String("\"variables.var\""), src_file, 26, 0, 26)
        );

        assert_eq!(l.next(), None);
    }

    #[test]
    fn test_comment() {
        let src_file = "Some file.st";
        let src = r#"// Hello
// World
(*
 Stuff
*)"#;
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), exp(Token::Comment("// Hello"), src_file, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::Comment("// World"), src_file, 9, 1, 0));
        assert_eq!(
            l.next(),
            exp(Token::Comment("(*\n Stuff\n*)"), src_file, 18, 2, 0)
        );

        assert_eq!(l.next(), None);
    }
}
