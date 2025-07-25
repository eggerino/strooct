#include "greatest/greatest.h"

SUITE(lexer_suite);

GREATEST_MAIN_DEFS();
int main(int argc, char **argv) {
    GREATEST_MAIN_BEGIN();

    RUN_SUITE(lexer_suite);

    GREATEST_MAIN_END();
}