#ifndef STROOCT_TOKEN_H_
#define STROOCT_TOKEN_H_

#include <stdbool.h>
#include <stddef.h>

#include "strooct/string.h"

typedef enum ST_TokenKind {
    ST_TOKEN_ILLEGAL = 0,

    // Literals
    ST_TOKEN_IDENTIFIER,
    ST_TOKEN_NUMBER,
    ST_TOKEN_STRING,
    ST_TOKEN_TIME,

    // Operators
    ST_TOKEN_PLUS,
    ST_TOKEN_MINUS,
    ST_TOKEN_ASTERISK,
    ST_TOKEN_SLASH,
    ST_TOKEN_ASSIGN,

    ST_TOKEN_EQUALS,
    ST_TOKEN_NOT_EQUALS,
    ST_TOKEN_GREATER_THAN,
    ST_TOKEN_GREATER_THAN_OR_EQUALS,
    ST_TOKEN_LESS_THAN,
    ST_TOKEN_LESS_THAN_OR_EQUALS,

    // Delimiters
    ST_TOKEN_DOT,
    ST_TOKEN_COMMA,
    ST_TOKEN_COLON,
    ST_TOKEN_SEMI_COLON,

    ST_TOKEN_LEFT_PARENTHESIS,
    ST_TOKEN_RIGHT_PARENTHESIS,
    ST_TOKEN_LEFT_BRACKET,
    ST_TOKEN_RIGHT_BRACKET,
    ST_TOKEN_LEFT_BRACE,
    ST_TOKEN_RIGHT_BRACE,

    // Keywords
    // Logic
    ST_TOKEN_NOT,
    ST_TOKEN_TRUE,
    ST_TOKEN_FALSE,
    ST_TOKEN_AND,
    ST_TOKEN_OR,
    ST_TOKEN_XOR,

    // Control flow
    ST_TOKEN_IF,
    ST_TOKEN_THEN,
    ST_TOKEN_ELSIF,
    ST_TOKEN_ELSE,
    ST_TOKEN_END_IF,
    ST_TOKEN_CASE,
    ST_TOKEN_OF,
    ST_TOKEN_END_CASE,
    ST_TOKEN_FOR,
    ST_TOKEN_TO,
    ST_TOKEN_BY,
    ST_TOKEN_DO,
    ST_TOKEN_END_FOR,
    ST_TOKEN_WHILE,
    ST_TOKEN_END_WHILE,

    // Program / Action / Function / Function block
    ST_TOKEN_PROGRAM,
    ST_TOKEN_END_PROGRAM,
    ST_TOKEN_EXIT,
    ST_TOKEN_ACTION,
    ST_TOKEN_END_ACTION,
    ST_TOKEN_FUNCTION,
    ST_TOKEN_END_FUNCTION,
    ST_TOKEN_FUNCTION_BLOCK,
    ST_TOKEN_END_FUNCTION_BLOCK,
    ST_TOKEN_RETURN,

    // Variable declarations
    ST_TOKEN_VAR,
    ST_TOKEN_VAR_INPUT,
    ST_TOKEN_VAR_OUTPUT,
    ST_TOKEN_CONSTANT,
    ST_TOKEN_END_VAR,

    // Type declarations
    ST_TOKEN_TYPE,
    ST_TOKEN_END_TYPE,
    ST_TOKEN_STRUCT,
    ST_TOKEN_END_STRUCT,
    ST_TOKEN_UNION,
    ST_TOKEN_END_UNION
} ST_TokenKind;

typedef struct ST_Token {
    ST_Str src_file;
    ST_Str lit;
    size_t pos;
    size_t line;
    size_t col;
    ST_TokenKind kind;
} ST_Token;

bool st_token_try_get_keyword(const char *str_ptr, ST_TokenKind *kind, size_t *lit_len);

#endif
