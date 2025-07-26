#include "strooct/string.h"

int st_str_cmp(ST_Str a, ST_Str b) {
    size_t min_len = a.len < b.len ? a.len : b.len;
    int cmp_result = strncmp(a.ptr, b.ptr, min_len);

    // When the lengths are equal, return the result of strncmp
    if (a.len == b.len) {
        return cmp_result;
    }

    // If the minimum comparable parts already differ, return their lexical order
    if (cmp_result) {
        return cmp_result;
    }

    // If the minimum comparable parts match, consider the shorter string lexically first
    return a.len - b.len;
}

bool st_str_equal(ST_Str a, ST_Str b) {
    if (a.len != b.len) {
        return false;
    }

    return strncmp(a.ptr, b.ptr, a.len) == 0;
}

ST_Str st_str_slice(ST_Str str, size_t offset, size_t len) {
    // When slicing outside the str return an empty str
    if (offset >= str.len || offset + len > str.len) {
        return (ST_Str){0};
    }

    return (ST_Str){.ptr = &str.ptr[offset], .len = len};
}
