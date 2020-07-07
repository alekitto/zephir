project(Google_tests)

# adding the Google_Tests_run target
add_executable(Google_Tests_run libzephir/PolicyTests.cpp libzephir/identity/RoleTests.cpp libzephir/identity/SubjectTests.cpp libzephir/identity/GroupTests.cpp)
target_link_libraries(Google_Tests_run libzephir pthread gtest gtest_main)
