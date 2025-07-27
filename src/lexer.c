#include "strooct/lexer.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>

#define CHAR_AT(l, i) (l).src.ptr[i]
#define CURRENT_CHAR(l) CHAR_AT(l, (l).pos)

#define EXCEEDS_AT(l, i) (i >= (l).src.len)
#define EXCEEDS(l) EXCEEDS_AT(l, (l).pos)

static bool next_token(ST_Lexer *l, ST_Token *t);
static size_t get_token(const ST_Lexer *l, ST_Token *t);
static bool try_get_operator(const ST_Lexer *l, ST_TokenKind *kind, size_t *lit_len);
static bool try_get_delimiter(char cur_char, ST_TokenKind *kind, size_t *lit_len);
static bool try_get_string(const ST_Lexer *l, size_t *lit_len);
static bool try_get_identifier(const ST_Lexer *l, size_t *lit_len);
static bool try_get_number(const ST_Lexer *l, size_t *lit_len);
static size_t get_whitespace_count(const ST_Lexer *l);
static bool advance(ST_Lexer *l, size_t n);

void st_lexer_init(ST_Str src_file, ST_Str src, ST_Lexer *l) {
    l->src = src;
    l->src_file = src_file;
    l->pos = 0;
    l->line = 0;
    l->col = 0;
}

bool st_lexer_next_token(ST_Lexer *l, ST_Token *t) {
    // Nothing to tokenize from
    if (l->src.ptr == NULL) {
        return false;
    }

    // Source is exhausted
    if (EXCEEDS(*l)) {
        l->src = (ST_Str){0};
        return false;
    }

    return next_token(l, t);
}

bool next_token(ST_Lexer *l, ST_Token *t) {
    advance(l, get_whitespace_count(l));

    if (EXCEEDS(*l)) {
        return false;
    }

    advance(l, get_token(l, t));
    return true;
}

size_t get_token(const ST_Lexer *l, ST_Token *t) {
    ST_TokenKind kind = ST_TOKEN_ILLEGAL;
    size_t token_lit_len = 0;
    
#define RETURN                          \
    t->src_file = l->src_file;          \
    t->lit.ptr = &CURRENT_CHAR(*l);     \
    t->lit.len = token_lit_len;         \
    t->pos = l->pos;                    \
    t->line = l->line;                  \
    t->col = l->col;                    \
    t->kind = kind;                     \
    return token_lit_len
    
    // Check keywords first
    if (st_token_try_get_keyword(&CURRENT_CHAR(*l), &kind, &token_lit_len)) {
        RETURN;
    }

    // Check literals
    if (try_get_identifier(l, &token_lit_len)) {
        kind = ST_TOKEN_IDENTIFIER;
        RETURN;
    }

    if (try_get_string(l, &token_lit_len)) {
        kind = ST_TOKEN_STRING;
        RETURN;
    }

    if (try_get_number(l, &token_lit_len)) {
        kind = ST_TOKEN_NUMBER;
        RETURN;
    }

    // Check operators before delimiters
    if (try_get_operator(l, &kind, &token_lit_len)) {
        RETURN;
    }

    // Check delimiter
    if (try_get_delimiter(CURRENT_CHAR(*l), &kind, &token_lit_len)) {
        RETURN;
    }

    // No correct token could be found. Consider the rest of the source string as an illegal token.
    kind = ST_TOKEN_ILLEGAL;
    token_lit_len = l->src.len - l->pos;
    RETURN;

#undef RETURN
}

bool try_get_operator(const ST_Lexer *l, ST_TokenKind *kind, size_t *lit_len) {
    char cur_char = CURRENT_CHAR(*l);

#define MATCH_CHAR(c, k) \
    if (cur_char == c) { \
        *kind = k;       \
        *lit_len = 1;    \
    }
#define MATCH_STR2(str, k)                         \
    if (strncmp(str, &CURRENT_CHAR(*l), 2) == 0) { \
        *kind = k;                                 \
        *lit_len = 2;                              \
    }

    MATCH_CHAR('+', ST_TOKEN_PLUS)
    else MATCH_CHAR('-', ST_TOKEN_MINUS)
    else MATCH_CHAR('*', ST_TOKEN_ASTERISK)
    else MATCH_CHAR('/', ST_TOKEN_SLASH)
    else MATCH_STR2(":=", ST_TOKEN_ASSIGN)
    else MATCH_CHAR('=', ST_TOKEN_EQUALS)
    else MATCH_STR2("<>", ST_TOKEN_NOT_EQUALS)
    else MATCH_STR2(">=", ST_TOKEN_GREATER_THAN_OR_EQUALS)  // check or equal before strict difference
    else MATCH_CHAR('>', ST_TOKEN_GREATER_THAN)
    else MATCH_STR2("<=", ST_TOKEN_LESS_THAN_OR_EQUALS)
    else MATCH_CHAR('<', ST_TOKEN_LESS_THAN)
    else {
        return false;
    }

#undef MATCH_CHAR
#undef MATCH_STR2

    return true;
}

bool try_get_delimiter(char cur_char, ST_TokenKind *kind, size_t *lit_len) {
#define CASE(c, k) \
    case c:        \
        *kind = k; \
        break

    switch (cur_char) {
        CASE('.', ST_TOKEN_DOT);
        CASE(',', ST_TOKEN_COMMA);
        CASE(':', ST_TOKEN_COLON);
        CASE(';', ST_TOKEN_SEMI_COLON);

        CASE('(', ST_TOKEN_LEFT_PARENTHESIS);
        CASE(')', ST_TOKEN_RIGHT_PARENTHESIS);
        CASE('[', ST_TOKEN_LEFT_BRACKET);
        CASE(']', ST_TOKEN_RIGHT_BRACKET);
        CASE('{', ST_TOKEN_LEFT_BRACE);
        CASE('}', ST_TOKEN_RIGHT_BRACE);

        default:
            return false;
    }

#undef CASE

    *lit_len = 1;
    return true;
}

bool try_get_string(const ST_Lexer *l, size_t *lit_len) {
    char start_quote = CURRENT_CHAR(*l);
    if (start_quote != '\'' && start_quote != '"') {
        return false;
    }

    for (size_t peak = l->pos + 1; peak < l->src.len; ++peak) {
        if (CHAR_AT(*l, peak) == start_quote) {
            *lit_len = peak - l->pos + 1;
            return true;
        }
    }

    // string is not terminated by quotes
    return false;
}

bool try_get_identifier(const ST_Lexer *l, size_t *lit_len) {
    // First character must be an "_" or a letter
    char first_char = CURRENT_CHAR(*l);
    if (first_char != '_' && !isalpha(first_char)) {
        return false;
    }

    size_t peak;
    for (peak = l->pos + 1; peak < l->src.len; ++peak) {
        // consecutive character must be "_"s, letters or numbers
        char cur_char = CHAR_AT(*l, peak);
        if (cur_char != '_' && !isalnum(cur_char)) {
            break;
        }
    }

    *lit_len = peak - l->pos;
    return true;
}

bool try_get_number(const ST_Lexer *l, size_t *lit_len) {
    char *start_ptr = &CURRENT_CHAR(*l);
    char *end_ptr = NULL;
    strtof(start_ptr, &end_ptr);

    // Current character does not start a number
    if (end_ptr <= start_ptr) {
        return false;
    }

    *lit_len = end_ptr - start_ptr;
    return true;
}

size_t get_whitespace_count(const ST_Lexer *l) {
    size_t count = 0;
    for (size_t i = l->pos; !EXCEEDS_AT(*l, i); ++i) {
        if (!isspace(CHAR_AT(*l, i))) {
            break;
        }
        count++;
    }
    return count;
}

bool advance(ST_Lexer *l, size_t n) {
    // Ensure n does not exceed the source string
    size_t max_n = l->src.len - l->pos;
    n = max_n < n ? max_n : n;

    for (size_t i = 0; i < n; ++i) {
        // Track new lines
        if (CURRENT_CHAR(*l) == '\n') {
            l->line++;
            l->col = 0;
        } else {
            l->col++;
        }

        l->pos++;
    }

    return n > 0;
}
