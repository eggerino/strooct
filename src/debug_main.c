#include <stdio.h>
#include "strooct/string.h"
#include "strooct/token.h"

int main(void) {
    ST_TokenKind kind = {0};
    int result = st_token_try_get_keyword("STRUCT", &kind);

    printf("Result=%d\n", result);
    printf("Kind=%d\n", kind);

    return 0;
}