cmake_minimum_required(VERSION 3.10)

project(strooct)
set(CMAKE_C_STANDARD 99)

# Main library
add_library(strooct
    "src/lexer.c"
    "src/string.c"
    "src/token.c"
)
target_compile_options(strooct PRIVATE -pedantic -Wall -Wextra -Werror)
target_include_directories(strooct PRIVATE "include")

# Debug program
add_executable(strooct_debug
    "src/debug_main.c"
)
target_compile_options(strooct_debug PRIVATE -pedantic -Wall -Wextra -Werror)
target_include_directories(strooct_debug PRIVATE "include")
target_link_libraries(strooct_debug strooct)

# Unit tests
add_executable(strooct_test
    "test/lexer.c"
    "test/main.c"
)
target_compile_options(strooct_test PRIVATE -pedantic -Wall -Wextra -Werror)
target_include_directories(strooct_test PRIVATE "include")
target_link_libraries(strooct_test strooct)
