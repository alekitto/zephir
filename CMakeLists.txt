cmake_minimum_required(VERSION 2.8.12)
project(zephir)

add_definitions("-std=c++17")

if(CMAKE_BUILD_TYPE MATCHES Debug)
    set(CMAKE_CXX_FLAGS --coverage)
endif()

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

add_library(libzephir STATIC libzephir/Policy.cpp libzephir/compiler/Compiler.cpp libzephir/util/util.cpp libzephir/MatchResult.cpp)
set_target_properties(libzephir PROPERTIES PREFIX "")

add_executable(zephir main.cpp)
target_link_libraries(zephir libzephir ${CONAN_LIBS})

add_subdirectory(tests)
