#include <stdio.h>
#include "strooct/string.h"
#include "strooct/token.h"

int main(void) {
    char *source = "STRUCTERU";
    ST_TokenKind kind;
    size_t literal_len;
    int result = st_token_try_get_keyword(source, &kind, &literal_len);

    printf("source=%s\n", source);
    printf("result=%d\n", result);
    printf("kind=%d\n", kind);
    printf("literal=%.*s\n", (int)literal_len, source);

    return 0;
}