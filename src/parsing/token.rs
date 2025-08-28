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

#[derive(Clone, Debug, PartialEq)]
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
