use crate::parsing::token::{MarkedToken, Marker, NumberValue, TimeValue, Token};
use nom::number::complete::double;

fn numeric_len(s: &str) -> Option<usize> {
    double::<_, ()>(s).map(|(r, _)| s.len() - r.len()).ok()
}

pub struct Lexer<'a> {
    src: &'a str,
    marker: Marker<'a>,
}

type TokenResult<'a> = Option<(Token<'a>, usize)>;

impl<'a> Lexer<'a> {
    pub fn create(src_file: &'a str, src: &'a str) -> Self {
        Self {
            src,
            marker: Marker::create(src_file, src),
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
        self.get_keyword_token()
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

    fn get_keyword_token(&self) -> TokenResult<'a> {
        match self.src {
            // Logic
            s if s.starts_with("NOT") => Some((Token::Not, 3)),
            s if s.starts_with("TRUE") => Some((Token::True, 4)),
            s if s.starts_with("FALSE") => Some((Token::False, 5)),
            s if s.starts_with("AND") => Some((Token::And, 3)),
            s if s.starts_with("OR") => Some((Token::Or, 2)),
            s if s.starts_with("XOR") => Some((Token::Xor, 3)),

            // Control flow
            s if s.starts_with("IF") => Some((Token::If, 2)),
            s if s.starts_with("THEN") => Some((Token::Then, 4)),
            s if s.starts_with("ELSIF") => Some((Token::Elsif, 5)),
            s if s.starts_with("ELSE") => Some((Token::Else, 4)),
            s if s.starts_with("END_IF") => Some((Token::EndIf, 6)),
            s if s.starts_with("CASE") => Some((Token::Case, 4)),
            s if s.starts_with("OF") => Some((Token::Of, 2)),
            s if s.starts_with("END_CASE") => Some((Token::EndCase, 8)),
            s if s.starts_with("FOR") => Some((Token::For, 3)),
            s if s.starts_with("TO") => Some((Token::To, 2)),
            s if s.starts_with("BY") => Some((Token::By, 2)),
            s if s.starts_with("DO") => Some((Token::Do, 2)),
            s if s.starts_with("END_FOR") => Some((Token::EndFor, 7)),
            s if s.starts_with("WHILE") => Some((Token::While, 5)),
            s if s.starts_with("END_WHILE") => Some((Token::EndWhile, 9)),

            // Program / Action / Function / Function block
            s if s.starts_with("PROGRAM") => Some((Token::Program, 7)),
            s if s.starts_with("END_PROGRAM") => Some((Token::EndProgram, 11)),
            s if s.starts_with("EXIT") => Some((Token::Exit, 4)),
            s if s.starts_with("ACTION") => Some((Token::Action, 6)),
            s if s.starts_with("END_ACTION") => Some((Token::EndAction, 10)),
            s if s.starts_with("FUNCTION_BLOCK") => Some((Token::FunctionBlock, 14)),
            s if s.starts_with("END_FUNCTION_BLOCK") => Some((Token::EndFunctionBlock, 18)),
            s if s.starts_with("FUNCTION") => Some((Token::Function, 8)),
            s if s.starts_with("END_FUNCTION") => Some((Token::EndFunction, 12)),
            s if s.starts_with("RETURN") => Some((Token::Return, 6)),

            // Variable declarations
            s if s.starts_with("VAR_INPUT") => Some((Token::VarInput, 9)),
            s if s.starts_with("VAR_OUTPUT") => Some((Token::VarOutput, 10)),
            s if s.starts_with("VAR") => Some((Token::Var, 3)),
            s if s.starts_with("CONSTANT") => Some((Token::Constant, 8)),
            s if s.starts_with("END_VAR") => Some((Token::EndVar, 7)),

            // Type declarations
            s if s.starts_with("TYPE") => Some((Token::Type, 4)),
            s if s.starts_with("END_TYPE") => Some((Token::EndType, 8)),
            s if s.starts_with("STRUCT") => Some((Token::Struct, 6)),
            s if s.starts_with("END_STRUCT") => Some((Token::EndStruct, 10)),
            s if s.starts_with("UNION") => Some((Token::Union, 5)),
            s if s.starts_with("END_UNION") => Some((Token::EndUnion, 9)),
            _ => None,
        }
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

        let mut data = TimeValue {
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
                Ok(int) => Some((Token::Number(&self.src[..n], NumberValue::Int(int)), n)),
                _ => Some((
                    Token::Number(
                        &self.src[..n],
                        NumberValue::Float(
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
            s if s.starts_with("%") => Some((Token::Percent, 1)),
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

    fn marker(
        src_file: &'static str,
        src: &'static str,
        pos: usize,
        line: usize,
        col: usize,
    ) -> Marker<'static> {
        let mut marker = Marker::create(src_file, src);
        marker.set(pos, line, col);
        marker
    }

    fn exp(
        t: Token<'static>,
        src_file: &'static str,
        src: &'static str,
        pos: usize,
        line: usize,
        col: usize,
    ) -> Option<MarkedToken<'static>> {
        Some(t.mark(marker(src_file, src, pos, line, col)))
    }

    #[test]
    fn test_whitespace() {
        let src_file = "Some file.st";
        let src = " \t\n\r\n  ";
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), None);
        assert_eq!(l.marker, marker(src_file, src, 7, 2, 2));
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

        assert_eq!(l.next(), exp(Token::Not, src_file, src, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::True, src_file, src, 4, 1, 0));
        assert_eq!(l.next(), exp(Token::False, src_file, src, 9, 2, 0));
        assert_eq!(l.next(), exp(Token::And, src_file, src, 15, 3, 0));
        assert_eq!(l.next(), exp(Token::Or, src_file, src, 19, 4, 0));
        assert_eq!(l.next(), exp(Token::Xor, src_file, src, 22, 5, 0));
        assert_eq!(l.next(), exp(Token::If, src_file, src, 26, 6, 0));
        assert_eq!(l.next(), exp(Token::Then, src_file, src, 29, 7, 0));
        assert_eq!(l.next(), exp(Token::Elsif, src_file, src, 34, 8, 0));
        assert_eq!(l.next(), exp(Token::Else, src_file, src, 40, 9, 0));
        assert_eq!(l.next(), exp(Token::EndIf, src_file, src, 45, 10, 0));
        assert_eq!(l.next(), exp(Token::Case, src_file, src, 52, 11, 0));
        assert_eq!(l.next(), exp(Token::Of, src_file, src, 57, 12, 0));
        assert_eq!(l.next(), exp(Token::EndCase, src_file, src, 60, 13, 0));
        assert_eq!(l.next(), exp(Token::For, src_file, src, 69, 14, 0));
        assert_eq!(l.next(), exp(Token::To, src_file, src, 73, 15, 0));
        assert_eq!(l.next(), exp(Token::By, src_file, src, 76, 16, 0));
        assert_eq!(l.next(), exp(Token::Do, src_file, src, 79, 17, 0));
        assert_eq!(l.next(), exp(Token::EndFor, src_file, src, 82, 18, 0));
        assert_eq!(l.next(), exp(Token::While, src_file, src, 90, 19, 0));
        assert_eq!(l.next(), exp(Token::EndWhile, src_file, src, 96, 20, 0));
        assert_eq!(l.next(), exp(Token::Program, src_file, src, 106, 21, 0));
        assert_eq!(l.next(), exp(Token::EndProgram, src_file, src, 114, 22, 0));
        assert_eq!(l.next(), exp(Token::Exit, src_file, src, 126, 23, 0));
        assert_eq!(l.next(), exp(Token::Action, src_file, src, 131, 24, 0));
        assert_eq!(l.next(), exp(Token::EndAction, src_file, src, 138, 25, 0));
        assert_eq!(l.next(), exp(Token::Function, src_file, src, 149, 26, 0));
        assert_eq!(l.next(), exp(Token::EndFunction, src_file, src, 158, 27, 0));
        assert_eq!(
            l.next(),
            exp(Token::FunctionBlock, src_file, src, 171, 28, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::EndFunctionBlock, src_file, src, 186, 29, 0)
        );
        assert_eq!(l.next(), exp(Token::Return, src_file, src, 205, 30, 0));
        assert_eq!(l.next(), exp(Token::Var, src_file, src, 212, 31, 0));
        assert_eq!(l.next(), exp(Token::VarInput, src_file, src, 216, 32, 0));
        assert_eq!(l.next(), exp(Token::VarOutput, src_file, src, 226, 33, 0));
        assert_eq!(l.next(), exp(Token::Constant, src_file, src, 237, 34, 0));
        assert_eq!(l.next(), exp(Token::EndVar, src_file, src, 246, 35, 0));
        assert_eq!(l.next(), exp(Token::Type, src_file, src, 254, 36, 0));
        assert_eq!(l.next(), exp(Token::EndType, src_file, src, 259, 37, 0));
        assert_eq!(l.next(), exp(Token::Struct, src_file, src, 268, 38, 0));
        assert_eq!(l.next(), exp(Token::EndStruct, src_file, src, 275, 39, 0));
        assert_eq!(l.next(), exp(Token::Union, src_file, src, 286, 40, 0));
        assert_eq!(l.next(), exp(Token::EndUnion, src_file, src, 292, 41, 0));

        assert_eq!(l.next(), None);
    }

    #[test]
    fn test_operators() {
        let src_file = "Some file.st";
        let src = r#"+
-
*
/
%
:=
=
<>
>
>=
<
<="#;
        let mut l = Lexer::create(src_file, src);

        assert_eq!(l.next(), exp(Token::Plus, src_file, src, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::Minus, src_file, src, 2, 1, 0));
        assert_eq!(l.next(), exp(Token::Asterisk, src_file, src, 4, 2, 0));
        assert_eq!(l.next(), exp(Token::Slash, src_file, src, 6, 3, 0));
        assert_eq!(l.next(), exp(Token::Percent, src_file, src, 8, 4, 0));
        assert_eq!(l.next(), exp(Token::Assign, src_file, src, 10, 5, 0));
        assert_eq!(l.next(), exp(Token::Equals, src_file, src, 13, 6, 0));
        assert_eq!(l.next(), exp(Token::NotEquals, src_file, src, 15, 7, 0));
        assert_eq!(l.next(), exp(Token::GreaterThan, src_file, src, 18, 8, 0));
        assert_eq!(
            l.next(),
            exp(Token::GreaterThanOrEquals, src_file, src, 20, 9, 0)
        );
        assert_eq!(l.next(), exp(Token::LessThan, src_file, src, 23, 10, 0));
        assert_eq!(
            l.next(),
            exp(Token::LessThanOrEquals, src_file, src, 25, 11, 0)
        );

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

        assert_eq!(l.next(), exp(Token::Dot, src_file, src, 0, 0, 0));
        assert_eq!(l.next(), exp(Token::Comma, src_file, src, 2, 1, 0));
        assert_eq!(l.next(), exp(Token::Colon, src_file, src, 4, 2, 0));
        assert_eq!(l.next(), exp(Token::SemiColon, src_file, src, 6, 3, 0));
        assert_eq!(
            l.next(),
            exp(Token::LeftParenthesis, src_file, src, 8, 4, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::RightParenthesis, src_file, src, 10, 5, 0)
        );
        assert_eq!(l.next(), exp(Token::LeftBracket, src_file, src, 12, 6, 0));
        assert_eq!(l.next(), exp(Token::RightBracket, src_file, src, 14, 7, 0));
        assert_eq!(l.next(), exp(Token::LeftBrace, src_file, src, 16, 8, 0));
        assert_eq!(l.next(), exp(Token::RightBrace, src_file, src, 18, 9, 0));

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
            exp(Token::String("\"Hello World'\""), src_file, src, 0, 0, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::String("'Hello World\"'"), src_file, src, 15, 1, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::Identifier("Identier_123"), src_file, src, 30, 2, 0)
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Identifier("_private_Identifier321"),
                src_file,
                src,
                43,
                3,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(Token::Identifier("_123Identifier"), src_file, src, 66, 4, 0)
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("1", NumberValue::Int(1)),
                src_file,
                src,
                81,
                5,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("234", NumberValue::Int(234)),
                src_file,
                src,
                83,
                6,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("1.23", NumberValue::Float(1.23)),
                src_file,
                src,
                87,
                7,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("1.23e8", NumberValue::Float(1.23e8)),
                src_file,
                src,
                92,
                8,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("2.34E+3", NumberValue::Float(2.34e3)),
                src_file,
                src,
                99,
                9,
                0
            )
        );
        assert_eq!(
            l.next(),
            exp(
                Token::Number("4.21e-4", NumberValue::Float(4.21e-4)),
                src_file,
                src,
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
                    TimeValue {
                        days: 0,
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milli_seconds: 0
                    }
                ),
                src_file,
                src,
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
                    TimeValue {
                        days: 1,
                        hours: 1,
                        minutes: 1,
                        seconds: 1,
                        milli_seconds: 1
                    }
                ),
                src_file,
                src,
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
                    TimeValue {
                        days: 1,
                        hours: 1,
                        minutes: 1,
                        seconds: 1,
                        milli_seconds: 1
                    }
                ),
                src_file,
                src,
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
                    TimeValue {
                        days: 0,
                        hours: 0,
                        minutes: 1,
                        seconds: 0,
                        milli_seconds: 1
                    }
                ),
                src_file,
                src,
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

        assert_eq!(
            l.next(),
            exp(Token::Pragma("#include"), src_file, src, 0, 0, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::String("\"header.h\""), src_file, src, 9, 0, 9)
        );
        assert_eq!(
            l.next(),
            exp(Token::Pragma("#load"), src_file, src, 20, 0, 20)
        );
        assert_eq!(
            l.next(),
            exp(Token::String("\"variables.var\""), src_file, src, 26, 0, 26)
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

        assert_eq!(
            l.next(),
            exp(Token::Comment("// Hello"), src_file, src, 0, 0, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::Comment("// World"), src_file, src, 9, 1, 0)
        );
        assert_eq!(
            l.next(),
            exp(Token::Comment("(*\n Stuff\n*)"), src_file, src, 18, 2, 0)
        );

        assert_eq!(l.next(), None);
    }
}
