#ifndef STROOCT_STRING_H_
#define STROOCT_STRING_H_

#include <stdbool.h>
#include <stddef.h>

#define ST_STRING_LITERAL(str) {str, sizeof(str) - 1}

typedef struct {
    char *ptr;
    size_t len;
} ST_String;

int st_string_cmp(ST_String str1, ST_String str2);
bool st_string_equals(ST_String str1, ST_String str2);

#endif
