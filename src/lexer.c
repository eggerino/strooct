#include "strooct/lexer.h"

#include <ctype.h>

#define CHAR_AT(l, i) (l).src.ptr[i]
#define CURRENT_CHAR(l) CHAR_AT(l, (l).pos)

#define EXCEEDS_AT(l, i) (i >= (l).src.len)
#define EXCEEDS(l) EXCEEDS_AT(l, (l).pos)

static bool next_token(ST_Lexer *l, ST_Token *t);
static size_t get_token(const ST_Lexer *l, ST_Token *t);
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

    /* No correct token could be found. Consider the rest of the source string as an illegal token. */
    kind = ST_TOKEN_ILLEGAL;
    token_lit_len = l->src.len - l->pos;
    RETURN_GET_TOKEN;
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
