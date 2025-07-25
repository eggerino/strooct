#ifndef STROOCT_LEXER_H_
#define STROOCT_LEXER_H_

#include <stdbool.h>
#include <stddef.h>

#include "strooct/string.h"
#include "strooct/token.h"

typedef struct {
    ST_String src;
    ST_String src_file;
    size_t pos;
    size_t line;
    size_t col;
} ST_Lexer;

void st_lexer_init(ST_String src_file, ST_String src, ST_Lexer *l);
bool st_lexer_next_token(ST_Lexer *l, ST_Token *t);

#endif
