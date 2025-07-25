#include "strooct/lexer.h"

#include <ctype.h>
#include <string.h>

#define CHAR_AT(l, i) (l).src.ptr[i]
#define CURRENT_CHAR(l) CHAR_AT(l, (l).pos)

#define EXCEEDS_AT(l, i) (i >= (l).src.len)
#define EXCEEDS(l) EXCEEDS_AT(l, (l).pos)

static bool next_token(ST_Lexer *l, ST_Token *t);
static size_t get_token(const ST_Lexer *l, ST_Token *t);
static bool try_get_operator(const ST_Lexer *l, ST_TokenKind *kind, size_t *lit_len);
static bool try_get_delimiter(char cur_char, ST_TokenKind *kind, size_t *lit_len);
static size_t get_whitespace_count(const ST_Lexer *l);
static bool advance(ST_Lexer *l, size_t n);

void st_lexer_init(ST_String src_file, ST_String src, ST_Lexer *l) {
    l->src = src;
    l->src_file = src_file;
    l->pos = 0;
    l->line = 0;
    l->col = 0;
}

bool st_lexer_next_token(ST_Lexer *l, ST_Token *t) {
    /* Nothing to tokenize from */
    if (l->src.ptr == NULL) {
        return false;
    }

    /* Source is exhausted */
    if (EXCEEDS(*l)) {
        ST_String empty = {0};
        l->src = empty;
        return false;
    }

    return next_token(l, t);
}

bool next_token(ST_Lexer *l, ST_Token *t) {
    size_t len = get_whitespace_count(l);
    advance(l, len);

    if (EXCEEDS(*l)) {
        return false;
    }

    len = get_token(l, t);
    advance(l, len);
    return true;
}

#define RETURN_GET_TOKEN                \
    t->src_file = l->src_file;          \
    t->literal.ptr = &CURRENT_CHAR(*l); \
    t->literal.len = token_lit_len;     \
    t->pos = l->pos;                    \
    t->line = l->line;                  \
    t->col = l->col;                    \
    t->kind = kind;                     \
    return token_lit_len

size_t get_token(const ST_Lexer *l, ST_Token *t) {
    ST_TokenKind kind = ST_TOKEN_ILLEGAL;
    size_t token_lit_len = 0;
    /* Check keywords first */
    if (st_token_try_get_keyword(&CURRENT_CHAR(*l), &kind, &token_lit_len)) {
        RETURN_GET_TOKEN;
    }

    /* Check operators before delimiters */
    if (try_get_operator(l, &kind, &token_lit_len)) {
        RETURN_GET_TOKEN;
    }

    /* Check delimiter */
    if (try_get_delimiter(CURRENT_CHAR(*l), &kind, &token_lit_len)) {
        RETURN_GET_TOKEN;
    }

    /* No correct token could be found. Consider the rest of the source string as an illegal token. */
    kind = ST_TOKEN_ILLEGAL;
    token_lit_len = l->src.len - l->pos;
    RETURN_GET_TOKEN;
}

#define OPERATOR_1(c, k) \
    if (cur_char == c) { \
        *kind = k;       \
        *lit_len = 1;    \
    }
#define OPERATOR_2(str, k)                         \
    if (strncmp(str, &CURRENT_CHAR(*l), 2) == 0) { \
        *kind = k;                                 \
        *lit_len = 2;                              \
    }
bool try_get_operator(const ST_Lexer *l, ST_TokenKind *kind, size_t *lit_len) {
    char cur_char = CURRENT_CHAR(*l);

    OPERATOR_1('+', ST_TOKEN_PLUS)
    else OPERATOR_1('-', ST_TOKEN_MINUS)
    else OPERATOR_1('*', ST_TOKEN_ASTERISK)
    else OPERATOR_1('/', ST_TOKEN_SLASH)
    else OPERATOR_2(":=", ST_TOKEN_ASSIGN)
    else OPERATOR_1('=', ST_TOKEN_EQUALS)
    else OPERATOR_2("<>", ST_TOKEN_NOT_EQUALS)
    else OPERATOR_2(">=", ST_TOKEN_GREATER_THAN_OR_EQUALS)  /* check or equal before strict difference */
    else OPERATOR_1('>', ST_TOKEN_GREATER_THAN)
    else OPERATOR_2("<=", ST_TOKEN_LESS_THAN_OR_EQUALS)
    else OPERATOR_1('<', ST_TOKEN_LESS_THAN)
    else {
        return false;
    }

    return true;
}

#define DELIMITER_CASE(c, k) \
    case c:                  \
        *kind = k;           \
        break
bool try_get_delimiter(char cur_char, ST_TokenKind *kind, size_t *lit_len) {
    switch (cur_char) {
        DELIMITER_CASE('.', ST_TOKEN_DOT);
        DELIMITER_CASE(',', ST_TOKEN_COMMA);
        DELIMITER_CASE(':', ST_TOKEN_COLON);
        DELIMITER_CASE(';', ST_TOKEN_SEMI_COLON);

        DELIMITER_CASE('(', ST_TOKEN_LEFT_PARENTHESIS);
        DELIMITER_CASE(')', ST_TOKEN_RIGHT_PARENTHESIS);
        DELIMITER_CASE('[', ST_TOKEN_LEFT_BRACKET);
        DELIMITER_CASE(']', ST_TOKEN_RIGHT_BRACKET);
        DELIMITER_CASE('{', ST_TOKEN_LEFT_BRACE);
        DELIMITER_CASE('}', ST_TOKEN_RIGHT_BRACE);

        default:
            return false;
    }

    *lit_len = 1;
    return true;
}

size_t get_whitespace_count(const ST_Lexer *l) {
    size_t count = 0, i;
    for (i = l->pos; !EXCEEDS_AT(*l, i); ++i) {
        if (!isspace(CHAR_AT(*l, i))) {
            break;
        }
        count++;
    }
    return count;
}

bool advance(ST_Lexer *l, size_t n) {
    /* Ensure n does not exceed the source string */
    size_t max_n, i;
    max_n = l->src.len - l->pos;
    n = max_n < n ? max_n : n;

    for (i = 0; i < n; ++i) {
        /* Track new lines */
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
