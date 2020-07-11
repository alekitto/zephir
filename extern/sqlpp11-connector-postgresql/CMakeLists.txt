cmake_minimum_required(VERSION 3.11 FATAL_ERROR)
include(FetchContent)

FetchContent_Declare(
    extern_sqlpp11_connector_postgresql
    GIT_REPOSITORY https://github.com/matthijs/sqlpp11-connector-postgresql.git
    GIT_TAG        d48039db9874c3d6a9e01130fd48ed538ab2b167
)

FetchContent_GetProperties(extern_sqlpp11_connector_postgresql)
if(NOT extern_sqlpp11_connector_postgresql_POPULATED)
    FetchContent_Populate(extern_sqlpp11_connector_postgresql)

    add_library(sqlpp11-connector-postgresql STATIC
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/bind_result.cpp
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/connection.cpp
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/exception.cpp
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/prepared_statement.cpp
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/detail/connection_handle.cpp
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/detail/prepared_statement_handle.cpp
        ${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/src/result.cpp
    )
endif()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -I${extern_sqlpp11_connector_postgresql_SOURCE_DIR}/include")