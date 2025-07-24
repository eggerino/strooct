cmake_minimum_required(VERSION 3.10)

project(strooct)
set(CMAKE_C_STANDARD 90)

add_library(strooct
    "src/string.c"
)
target_compile_options(strooct PRIVATE -pedantic -Wall -Wextra -Werror)
target_include_directories(strooct PRIVATE "include")

add_executable(strooct_debug
    "src/debug_main.c"
)
target_compile_options(strooct_debug PRIVATE -pedantic -Wall -Wextra -Werror)
target_include_directories(strooct_debug PRIVATE "include")
target_link_libraries(strooct_debug strooct)

# add_executable(test
#     "src/main.c"
#     "src/info.c"
#     "src/interpreter.c"
#     "src/io.c"
#     "src/parser.c"
#     "src/repl.c"
#     "src/runtime.c"
#     "src/stagma.c"
# )
# target_link_libraries(stagma m) # Link math.h

# add_executable(stagmac
#     "src/ast.c"    
#     "src/compiler.c"
#     "src/info.c"
#     "src/io.c"
#     "src/parser.c"
#     "src/stagmac.c"
# )
# target_compile_options(stagmac PRIVATE -pedantic -Wall -Wextra -Werror)

# add_executable(dynamic-array-test "src/dynamic_array.test.c")