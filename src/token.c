#include "strooct/token.h"

#include <string.h>
#include "strooct/string.h"

#define KEYWORD_MAPPING_LEN 42
#define MAPPING(kind, str) {kind, {str, sizeof(str) - 1}}
static struct {
    ST_TokenKind kind;
    ST_String literal;
} keyword_mapping[KEYWORD_MAPPING_LEN] = {
    /* Logic */
    MAPPING(ST_TOKEN_NOT, "NOT"),
    MAPPING(ST_TOKEN_TRUE, "TRUE"),
    MAPPING(ST_TOKEN_FALSE, "FALSE"),
    MAPPING(ST_TOKEN_AND, "AND"),
    MAPPING(ST_TOKEN_OR, "OR"),
    MAPPING(ST_TOKEN_XOR, "XOR"),

    /* Control flow */
    MAPPING(ST_TOKEN_IF, "IF"),
    MAPPING(ST_TOKEN_THEN, "THEN"),
    MAPPING(ST_TOKEN_ELSIF, "ELSIF"),
    MAPPING(ST_TOKEN_ELSE, "ELSE"),
    MAPPING(ST_TOKEN_END_IF, "END_IF"),
    MAPPING(ST_TOKEN_CASE, "CASE"),
    MAPPING(ST_TOKEN_OF, "OF"),
    MAPPING(ST_TOKEN_END_CASE, "END_CASE"),
    MAPPING(ST_TOKEN_FOR, "FOR"),
    MAPPING(ST_TOKEN_TO, "TO"),
    MAPPING(ST_TOKEN_BY, "BY"),
    MAPPING(ST_TOKEN_DO, "DO"),
    MAPPING(ST_TOKEN_END_FOR, "END_FOR"),
    MAPPING(ST_TOKEN_WHILE, "WHILE"),
    MAPPING(ST_TOKEN_END_WHILE, "END_WHILE"),

    /* Program / Action / Function / Function block */
    MAPPING(ST_TOKEN_PROGRAM, "PROGRAM"),
    MAPPING(ST_TOKEN_END_PROGRAM, "END_PROGRAM"),
    MAPPING(ST_TOKEN_EXIT, "EXIT"),
    MAPPING(ST_TOKEN_ACTION, "ACTION"),
    MAPPING(ST_TOKEN_END_ACTION, "END_ACTION"),
    MAPPING(ST_TOKEN_FUNCTION_BLOCK, "FUNCTION_BLOCK"), /* Make sure "FUNCTION_BLOCK" is checked before "FUNCTION" */
    MAPPING(ST_TOKEN_END_FUNCTION_BLOCK, "END_FUNCTION_BLOCK"),
    MAPPING(ST_TOKEN_FUNCTION, "FUNCTION"),
    MAPPING(ST_TOKEN_END_FUNCTION, "END_FUNCTION"),
    MAPPING(ST_TOKEN_RETURN, "RETURN"),

    /* Variable declarations */
    MAPPING(ST_TOKEN_VAR, "VAR"),
    MAPPING(ST_TOKEN_VAR_INPUT, "VAR_INPUT"),
    MAPPING(ST_TOKEN_VAR_OUTPUT, "VAR_OUTPUT"),
    MAPPING(ST_TOKEN_CONSTANT, "CONSTANT"),
    MAPPING(ST_TOKEN_END_VAR, "END_VAR"),

    /* Type declarations */
    MAPPING(ST_TOKEN_TYPE, "TYPE"),
    MAPPING(ST_TOKEN_END_TYPE, "END_TYPE"),
    MAPPING(ST_TOKEN_STRUCT, "STRUCT"),
    MAPPING(ST_TOKEN_END_STRUCT, "END_STRUCT"),
    MAPPING(ST_TOKEN_UNION, "UNION"),
    MAPPING(ST_TOKEN_END_UNION, "END_UNION"),
};

bool st_token_try_get_keyword(const char *str_ptr, ST_TokenKind *kind, size_t *literal_len) {
    int i;
    for (i = 0; i < KEYWORD_MAPPING_LEN; ++i) {
        /*
        On match return the corresponding token kind
        First match wins
        if one keyword starts with another keyword, ensure the longer one occures first,
        else the shorter one will always get picked
        */
       size_t len = keyword_mapping[i].literal.len;
        if (strncmp(str_ptr, keyword_mapping[i].literal.ptr, len) == 0) {
            *kind = keyword_mapping[i].kind;
            *literal_len = len;
            return true;
        }
    }

    /* No matching keyword found */
    return false;
}
