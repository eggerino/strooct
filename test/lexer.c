#include "strooct/lexer.h"

#include <string.h>

#include "greatest/greatest.h"

#define ASSERT_NEXT_TOKEN(exp_kind, exp_pos, exp_line, exp_col, exp_lit, actual) \
    ASSERT(st_lexer_next_token(&l, &t));                                         \
    ASSERT(exp_kind == actual.kind);                                             \
    ASSERT(exp_pos == actual.pos);                                               \
    ASSERT(exp_line == actual.line);                                             \
    ASSERT(exp_col == actual.col);                                               \
    ASSERT(st_str_equal(ST_STR_LIT(exp_lit), actual.lit))

TEST lexer_test(void) {
    ST_Str src_file = ST_STR_LIT("Some file.st");
    ST_Str src = ST_STR_LIT(
        // whitespace noise at the start
        " \t\r\n\n  "

        // Keywords
        // Logic
        "NOT\n"
        "TRUE\n"
        "FALSE\n"
        "AND\n"
        "OR\n"
        "XOR\n"

        // Control flow
        "IF\n"
        "THEN\n"
        "ELSIF\n"
        "ELSE\n"
        "END_IF\n"
        "CASE\n"
        "OF\n"
        "END_CASE\n"
        "FOR\n"
        "TO\n"
        "BY\n"
        "DO\n"
        "END_FOR\n"
        "WHILE\n"
        "END_WHILE\n"

        // Program / Action / Function / Function block
        "PROGRAM\n"
        "END_PROGRAM\n"
        "EXIT\n"
        "ACTION\n"
        "END_ACTION\n"
        "FUNCTION\n"
        "END_FUNCTION\n"
        "FUNCTION_BLOCK\n"
        "END_FUNCTION_BLOCK\n"
        "RETURN\n"

        // Variable declarations
        "VAR\n"
        "VAR_INPUT\n"
        "VAR_OUTPUT\n"
        "CONSTANT\n"
        "END_VAR\n"

        // Type declarations
        "TYPE\n"
        "END_TYPE\n"
        "STRUCT\n"
        "END_STRUCT\n"
        "UNION\n"
        "END_UNION\n"

        // Operators
        "+\n"
        "-\n"
        "*\n"
        "/\n"
        ":=\n"

        "=\n"
        "<>\n"
        ">\n"
        ">=\n"
        "<\n"
        "<=\n"

        // Delimiters
        ".\n"
        ",\n"
        ":\n"
        ";\n"

        "(\n"
        ")\n"
        "[\n"
        "]\n"
        "{\n"
        "}\n"
    
        // Literals
        "\"Hello World'\"\n"
        "'Hello World\"'\n"

        "Identier_123\n"
        "_private_Identifier321\n"
        "_123Identifier\n"

        "1\n"
        "+234\n"
        "-43\n"
        "1.23\n"
        "+2.34\n"
        "-4.21\n"
        "1.23e8\n"
        "+2.34E+3\n"
        "-4.21e-4\n"

        "T#1s\n"
        "T#1D1M1S1MS\n"
        "T#1d1m1s1ms\n"
        "T#1m1ms\n"
    );

    ST_Lexer l;
    ST_Token t;
    st_lexer_init(src_file, src, &l);

    ASSERT_NEXT_TOKEN(ST_TOKEN_NOT, 7, 2, 2, "NOT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TRUE, 11, 3, 0, "TRUE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_FALSE, 16, 4, 0, "FALSE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_AND, 22, 5, 0, "AND", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_OR, 26, 6, 0, "OR", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_XOR, 29, 7, 0, "XOR", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_IF, 33, 8, 0, "IF", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_THEN, 36, 9, 0, "THEN", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_ELSIF, 41, 10, 0, "ELSIF", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_ELSE, 47, 11, 0, "ELSE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_IF, 52, 12, 0, "END_IF", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_CASE, 59, 13, 0, "CASE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_OF, 64, 14, 0, "OF", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_CASE, 67, 15, 0, "END_CASE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_FOR, 76, 16, 0, "FOR", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TO, 80, 17, 0, "TO", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_BY, 83, 18, 0, "BY", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_DO, 86, 19, 0, "DO", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_FOR, 89, 20, 0, "END_FOR", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_WHILE, 97, 21, 0, "WHILE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_WHILE, 103, 22, 0, "END_WHILE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_PROGRAM, 113, 23, 0, "PROGRAM", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_PROGRAM, 121, 24, 0, "END_PROGRAM", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_EXIT, 133, 25, 0, "EXIT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_ACTION, 138, 26, 0, "ACTION", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_ACTION, 145, 27, 0, "END_ACTION", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_FUNCTION, 156, 28, 0, "FUNCTION", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_FUNCTION, 165, 29, 0, "END_FUNCTION", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_FUNCTION_BLOCK, 178, 30, 0, "FUNCTION_BLOCK", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_FUNCTION_BLOCK, 193, 31, 0, "END_FUNCTION_BLOCK", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_RETURN, 212, 32, 0, "RETURN", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_VAR, 219, 33, 0, "VAR", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_VAR_INPUT, 223, 34, 0, "VAR_INPUT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_VAR_OUTPUT, 233, 35, 0, "VAR_OUTPUT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_CONSTANT, 244, 36, 0, "CONSTANT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_VAR, 253, 37, 0, "END_VAR", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TYPE, 261, 38, 0, "TYPE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_TYPE, 266, 39, 0, "END_TYPE", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_STRUCT, 275, 40, 0, "STRUCT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_STRUCT, 282, 41, 0, "END_STRUCT", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_UNION, 293, 42, 0, "UNION", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_END_UNION, 299, 43, 0, "END_UNION", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_PLUS, 309, 44, 0, "+", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_MINUS, 311, 45, 0, "-", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_ASTERISK, 313, 46, 0, "*", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_SLASH, 315, 47, 0, "/", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_ASSIGN, 317, 48, 0, ":=", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_EQUALS, 320, 49, 0, "=", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NOT_EQUALS, 322, 50, 0, "<>", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_GREATER_THAN, 325, 51, 0, ">", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_GREATER_THAN_OR_EQUALS, 327, 52, 0, ">=", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_LESS_THAN, 330, 53, 0, "<", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_LESS_THAN_OR_EQUALS, 332, 54, 0, "<=", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_DOT, 335, 55, 0, ".", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_COMMA, 337, 56, 0, ",", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_COLON, 339, 57, 0, ":", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_SEMI_COLON, 341, 58, 0, ";", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_LEFT_PARENTHESIS, 343, 59, 0, "(", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_RIGHT_PARENTHESIS, 345, 60, 0, ")", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_LEFT_BRACKET, 347, 61, 0, "[", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_RIGHT_BRACKET, 349, 62, 0, "]", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_LEFT_BRACE, 351, 63, 0, "{", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_RIGHT_BRACE, 353, 64, 0, "}", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_STRING, 355, 65, 0, "\"Hello World'\"", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_STRING, 370, 66, 0, "'Hello World\"'", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_IDENTIFIER, 385, 67, 0, "Identier_123", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_IDENTIFIER, 398, 68, 0, "_private_Identifier321", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_IDENTIFIER, 421, 69, 0, "_123Identifier", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 436, 70, 0, "1", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 438, 71, 0, "+234", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 443, 72, 0, "-43", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 447, 73, 0, "1.23", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 452, 74, 0, "+2.34", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 458, 75, 0, "-4.21", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 464, 76, 0, "1.23e8", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 471, 77, 0, "+2.34E+3", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_NUMBER, 480, 78, 0, "-4.21e-4", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TIME, 489, 79, 0, "T#1s", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TIME, 494, 80, 0, "T#1D1M1S1MS", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TIME, 506, 81, 0, "T#1d1m1s1ms", t);
    ASSERT_NEXT_TOKEN(ST_TOKEN_TIME, 518, 82, 0, "T#1m1ms", t);

    ASSERT(!st_lexer_next_token(&l, &t));
    PASS();
}

SUITE(lexer_suite) { RUN_TEST(lexer_test); }
