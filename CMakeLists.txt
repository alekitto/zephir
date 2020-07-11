cmake_minimum_required(VERSION 3.11 FATAL_ERROR)
project(zephir)

SET(ENABLE_TESTS OFF CACHE BOOL "SQLPP11 Build unit tests")
set(CMAKE_MODULE_PATH ${CMAKE_CURRENT_LIST_DIR}/extern ${CMAKE_MODULE_PATH})

add_definitions("-std=c++17")

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -I${CMAKE_BINARY_DIR}/_deps/extern_sqlpp11_connector_postgresql-src/include -I${CMAKE_BINARY_DIR}/_deps/extern_valijson-src/include")
if(CMAKE_BUILD_TYPE MATCHES Debug)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} --coverage")
endif()

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -I${CONAN_INCLUDE_DIRS_NLOHMANN_JSON}/nlohmann")

add_subdirectory(extern)
add_library(libzephir STATIC libzephir/Policy.cpp libzephir/compiler/Compiler.cpp libzephir/util/util.cpp libzephir/MatchResult.cpp libzephir/storage/Manager.cpp libzephir/storage/sql/postgres/PostgresManager.cpp)
set_target_properties(libzephir PROPERTIES PREFIX "")

add_executable(zephir "")
target_sources(zephir PRIVATE
    main.cpp
    server/json-reader.cpp
    server/schemas/schemas.cpp
    server/schemas/allowed.json.cpp
    server/schemas/upsert_policy.json.cpp
    server/actions/allowed.cpp
    server/actions/upsertPolicies.cpp
)

target_link_libraries(zephir libzephir "sqlpp11-connector-postgresql" ${CONAN_LIBS})

add_subdirectory(tests)
