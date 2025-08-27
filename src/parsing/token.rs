#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Illegal,
    Comment(&'a str),

    // Literals
    Identifier(&'a str),
    Number(&'a str, NumberValue),
    String(&'a str),
    Time(&'a str, TimeValue),

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Assign,

    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,

    // Delimiters
    Dot,
    Comma,
    Colon,
    SemiColon,

    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    // Keywords
    // Logic
    Not,
    True,
    False,
    And,
    Or,
    Xor,

    // Control flow
    If,
    Then,
    Elsif,
    Else,
    EndIf,
    Case,
    Of,
    EndCase,
    For,
    To,
    By,
    Do,
    EndFor,
    While,
    EndWhile,

    // Program / Action / Function / Function block
    Program,
    EndProgram,
    Exit,
    Action,
    EndAction,
    Function,
    EndFunction,
    FunctionBlock,
    EndFunctionBlock,
    Return,

    // Variable declarations
    Var,
    VarInput,
    VarOutput,
    Constant,
    EndVar,

    // Type declarations
    Type,
    EndType,
    Struct,
    EndStruct,
    Union,
    EndUnion,

    // Addons
    Pragma(&'a str),
}

impl Token<'static> {
    pub fn from_keyword<'a>(src: &'a str) -> Option<(Self, usize)> {
        match src {
            // Logic
            s if s.starts_with("NOT") => Some((Self::Not, 3)),
            s if s.starts_with("TRUE") => Some((Self::True, 4)),
            s if s.starts_with("FALSE") => Some((Self::False, 5)),
            s if s.starts_with("AND") => Some((Self::And, 3)),
            s if s.starts_with("OR") => Some((Self::Or, 2)),
            s if s.starts_with("XOR") => Some((Self::Xor, 3)),

            // Control flow
            s if s.starts_with("IF") => Some((Self::If, 2)),
            s if s.starts_with("THEN") => Some((Self::Then, 4)),
            s if s.starts_with("ELSIF") => Some((Self::Elsif, 5)),
            s if s.starts_with("ELSE") => Some((Self::Else, 4)),
            s if s.starts_with("END_IF") => Some((Self::EndIf, 6)),
            s if s.starts_with("CASE") => Some((Self::Case, 4)),
            s if s.starts_with("OF") => Some((Self::Of, 2)),
            s if s.starts_with("END_CASE") => Some((Self::EndCase, 8)),
            s if s.starts_with("FOR") => Some((Self::For, 3)),
            s if s.starts_with("TO") => Some((Self::To, 2)),
            s if s.starts_with("BY") => Some((Self::By, 2)),
            s if s.starts_with("DO") => Some((Self::Do, 2)),
            s if s.starts_with("END_FOR") => Some((Self::EndFor, 7)),
            s if s.starts_with("WHILE") => Some((Self::While, 5)),
            s if s.starts_with("END_WHILE") => Some((Self::EndWhile, 9)),

            // Program / Action / Function / Function block
            s if s.starts_with("PROGRAM") => Some((Self::Program, 7)),
            s if s.starts_with("END_PROGRAM") => Some((Self::EndProgram, 11)),
            s if s.starts_with("EXIT") => Some((Self::Exit, 4)),
            s if s.starts_with("ACTION") => Some((Self::Action, 6)),
            s if s.starts_with("END_ACTION") => Some((Self::EndAction, 10)),
            s if s.starts_with("FUNCTION_BLOCK") => Some((Self::FunctionBlock, 14)),
            s if s.starts_with("END_FUNCTION_BLOCK") => Some((Self::EndFunctionBlock, 18)),
            s if s.starts_with("FUNCTION") => Some((Self::Function, 8)),
            s if s.starts_with("END_FUNCTION") => Some((Self::EndFunction, 12)),
            s if s.starts_with("RETURN") => Some((Self::Return, 6)),

            // Variable declarations
            s if s.starts_with("VAR_INPUT") => Some((Self::VarInput, 9)),
            s if s.starts_with("VAR_OUTPUT") => Some((Self::VarOutput, 10)),
            s if s.starts_with("VAR") => Some((Self::Var, 3)),
            s if s.starts_with("CONSTANT") => Some((Self::Constant, 8)),
            s if s.starts_with("END_VAR") => Some((Self::EndVar, 7)),

            // Type declarations
            s if s.starts_with("TYPE") => Some((Self::Type, 4)),
            s if s.starts_with("END_TYPE") => Some((Self::EndType, 8)),
            s if s.starts_with("STRUCT") => Some((Self::Struct, 6)),
            s if s.starts_with("END_STRUCT") => Some((Self::EndStruct, 10)),
            s if s.starts_with("UNION") => Some((Self::Union, 5)),
            s if s.starts_with("END_UNION") => Some((Self::EndUnion, 9)),
            _ => None,
        }
    }
}

impl<'a> Token<'a> {
    pub fn mark(self, marker: Marker<'a>) -> MarkedToken<'a> {
        MarkedToken {
            token: self,
            marker,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberValue {
    Int(usize),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub struct TimeValue {
    pub days: usize,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milli_seconds: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Marker<'a> {
    pos: usize,
    line: usize,
    col: usize,
    src_file: &'a str,
    src: &'a str,
}

impl<'a> Marker<'a> {
    pub fn create(src_file: &'a str, src: &'a str) -> Self {
        Self {
            pos: 0,
            line: 0,
            col: 0,
            src_file,
            src,
        }
    }

    pub fn set(&mut self, pos: usize, line: usize, col: usize) {
        self.pos = pos;
        self.line = line;
        self.col = col;
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.col += 1;
    }

    pub fn advance_new_line(&mut self) {
        self.pos += 1;
        self.line += 1;
        self.col = 0;
    }

    pub fn format_as_printable(&self) -> String {
        let src_line = self.get_line();

        format!(
            "{file}:{line}:{col}\n    |\n{line:^width$}| {src_line}\n    |  {arrow:>col$}",
            file = self.src_file,
            line = self.line + 1,
            col = self.col,
            width = 4,
            src_line = src_line,
            arrow = "^"
        )
    }

    fn get_line(&self) -> &str {
        let start_idx = self.src[..self.pos]
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '\n')
            .last()
            .map_or(0, |(i, _)| i + 1);

        let line = &self.src[start_idx..];

        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '\n')
            .next()
            .map_or(line, |(i, _)| &line[..i])
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkedToken<'a> {
    pub token: Token<'a>,
    pub marker: Marker<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_format_as_printable() {
        let marker = Marker {
            pos: 8,
            line: 2,
            col: 1,
            src_file: "Some file.st",
            src: "hel\nlo\nwor\nld",
        };

        assert_eq!(
            "Some file.st:3:1\n    |\n 3  | wor\n    |  ^",
            marker.format_as_printable()
        );
    }
}
