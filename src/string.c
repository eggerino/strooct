#include "strooct/string.h"

#include <string.h>

int st_string_cmp(ST_String str1, ST_String str2) {
    size_t min_len = str1.len < str2.len ? str1.len : str2.len;
    int cmp_result = strncmp(str1.ptr, str2.ptr, min_len);

    /* When the lengths are equal, return the result of strncmp */
    if (str1.len == str2.len) {
        return cmp_result;
    }

    /* If the minimum comparable parts already differ, return their lexical order */
    if (cmp_result) {
        return cmp_result;
    }

    /* If the minimum comparable parts match, consider the shorter string lexically first */
    return str1.len - str2.len;
}

bool st_string_equals(ST_String str1, ST_String str2) {
    if (str1.len != str2.len) {
        return false;
    }

    return strncmp(str1.ptr, str2.ptr, str1.len) == 0;
}
