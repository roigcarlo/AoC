cmake_minimum_required(VERSION 3.25)

project(AoC2023 CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")

# set(CMAKE_EXPERIMENTAL_CXX_MODULE_DYNDEP 1)
# set(CMAKE_EXPERIMENTAL_CXX_MODULE_CMAKE_API "aa1f7df0-828a-4fcd-9afc-2dc80491aca7")

# set(CMAKE_CXX_EXTENSIONS OFF)

# add_library(test_module)
# target_sources(test_module
#   PUBLIC
#     FILE_SET cxx_modules TYPE CXX_MODULES FILES
#     "modules/test_module.cpp"
# )

# add_executable(TestModules "test/test.cpp")

add_executable(main "d01/main.cpp")

# target_link_libraries(TestModules PUBLIC test_module)
