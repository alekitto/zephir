cmake_minimum_required(VERSION 3.11 FATAL_ERROR)
include(FetchContent)

FetchContent_Declare(
    extern_valijson
    GIT_REPOSITORY https://github.com/tristanpenman/valijson.git
    GIT_TAG        105e345a73b44267c21fb1f91f35983f68427e2f
)

FetchContent_GetProperties(extern_valijson)
if(NOT extern_valijson_POPULATED)
    FetchContent_Populate(extern_valijson)
endif()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -I${extern_valijson_SOURCE_DIR}/include")
