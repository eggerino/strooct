#include "strooct/token.h"

#include "strooct/string.h"

#define KEYWORD_COUNT 42
#define KEYWORD(kind, lit) {kind, {lit, sizeof(lit) - 1}}

static struct {
    ST_TokenKind kind;
    ST_Str lit;
} keywords[KEYWORD_COUNT] = {
    // Logic
    KEYWORD(ST_TOKEN_NOT, "NOT"),
    KEYWORD(ST_TOKEN_TRUE, "TRUE"),
    KEYWORD(ST_TOKEN_FALSE, "FALSE"),
    KEYWORD(ST_TOKEN_AND, "AND"),
    KEYWORD(ST_TOKEN_OR, "OR"),
    KEYWORD(ST_TOKEN_XOR, "XOR"),

    // Control flow
    KEYWORD(ST_TOKEN_IF, "IF"),
    KEYWORD(ST_TOKEN_THEN, "THEN"),
    KEYWORD(ST_TOKEN_ELSIF, "ELSIF"),
    KEYWORD(ST_TOKEN_ELSE, "ELSE"),
    KEYWORD(ST_TOKEN_END_IF, "END_IF"),
    KEYWORD(ST_TOKEN_CASE, "CASE"),
    KEYWORD(ST_TOKEN_OF, "OF"),
    KEYWORD(ST_TOKEN_END_CASE, "END_CASE"),
    KEYWORD(ST_TOKEN_FOR, "FOR"),
    KEYWORD(ST_TOKEN_TO, "TO"),
    KEYWORD(ST_TOKEN_BY, "BY"),
    KEYWORD(ST_TOKEN_DO, "DO"),
    KEYWORD(ST_TOKEN_END_FOR, "END_FOR"),
    KEYWORD(ST_TOKEN_WHILE, "WHILE"),
    KEYWORD(ST_TOKEN_END_WHILE, "END_WHILE"),

    // Program / Action / Function / Function block
    KEYWORD(ST_TOKEN_PROGRAM, "PROGRAM"),
    KEYWORD(ST_TOKEN_END_PROGRAM, "END_PROGRAM"),
    KEYWORD(ST_TOKEN_EXIT, "EXIT"),
    KEYWORD(ST_TOKEN_ACTION, "ACTION"),
    KEYWORD(ST_TOKEN_END_ACTION, "END_ACTION"),
    KEYWORD(ST_TOKEN_FUNCTION_BLOCK, "FUNCTION_BLOCK"),  // Make sure "FUNCTION_BLOCK" is checked before "FUNCTION"
    KEYWORD(ST_TOKEN_END_FUNCTION_BLOCK, "END_FUNCTION_BLOCK"),
    KEYWORD(ST_TOKEN_FUNCTION, "FUNCTION"),
    KEYWORD(ST_TOKEN_END_FUNCTION, "END_FUNCTION"),
    KEYWORD(ST_TOKEN_RETURN, "RETURN"),

    // Variable declarations
    KEYWORD(ST_TOKEN_VAR_INPUT, "VAR_INPUT"),
    KEYWORD(ST_TOKEN_VAR_OUTPUT, "VAR_OUTPUT"),
    KEYWORD(ST_TOKEN_VAR, "VAR"),  // MAke sure "VAR" is check after "VAR_INPUT" and "VAR_OUTPUT"
    KEYWORD(ST_TOKEN_CONSTANT, "CONSTANT"),
    KEYWORD(ST_TOKEN_END_VAR, "END_VAR"),

    // Type declarations
    KEYWORD(ST_TOKEN_TYPE, "TYPE"),
    KEYWORD(ST_TOKEN_END_TYPE, "END_TYPE"),
    KEYWORD(ST_TOKEN_STRUCT, "STRUCT"),
    KEYWORD(ST_TOKEN_END_STRUCT, "END_STRUCT"),
    KEYWORD(ST_TOKEN_UNION, "UNION"),
    KEYWORD(ST_TOKEN_END_UNION, "END_UNION"),
};

bool st_token_try_get_keyword(const char *str_ptr, ST_TokenKind *kind, size_t *lit_len) {
    // Adjust the size of the current str ptr and check for matches on the stored literals
    for (int i = 0; i < KEYWORD_COUNT; ++i) {
        if (strncmp(str_ptr, keywords[i].lit.ptr, keywords[i].lit.len) == 0) {
            *kind = keywords[i].kind;
            *lit_len = keywords[i].lit.len;
            return true;
        }
    }

    // No matching keyword found
    return false;
}
