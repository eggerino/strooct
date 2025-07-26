#ifndef STROOCT_STRING_H_
#define STROOCT_STRING_H_

#include <stdbool.h>
#include <stddef.h>
#include <string.h>

#define ST_STR_LIT(lit) ((ST_Str){lit, sizeof(lit) - 1})
#define ST_STR_CSTR(str) ((ST_Str){str, strlen(str)})

typedef struct ST_Str {
    char *ptr;
    size_t len;
} ST_Str;

int st_str_cmp(ST_Str a, ST_Str b);
bool st_str_equal(ST_Str a, ST_Str b);
ST_Str st_str_slice(ST_Str str, size_t offset, size_t len);

#endif
