#include <stdio.h>
#include "strooct/string.h"

int main(void) {
    ST_String str1, str2;
    
    ST_STRING_SET_LITERAL(str1, "Hello");
    ST_STRING_SET_LITERAL(str2, "Helloooo");

    printf("Compare %d\n", st_string_cmp(str1, str2));
    printf("Equals %d\n", st_string_equals(str1, str2));

    return 0;
}