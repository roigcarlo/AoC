cmake_minimum_required(VERSION 3.25)

project(AoC2023 CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libc++ -fexperimental-library" )

include_directories(utilities)

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

add_executable(d01 "d01/main.cpp")
add_executable(d02 "d02/main.cpp")
add_executable(d03 "d03/main.cpp")
add_executable(d04 "d04/main.cpp")
add_executable(d05 "d05/main.cpp")
add_executable(d06 "d06/main.cpp")
add_executable(d07 "d07/main.cpp")
add_executable(d08 "d08/main.cpp")

target_link_libraries(d01 PUBLIC tbb)
target_link_libraries(d02 PUBLIC tbb)
target_link_libraries(d03 PUBLIC tbb)
target_link_libraries(d04 PUBLIC tbb)
target_link_libraries(d05 PUBLIC tbb)
target_link_libraries(d06 PUBLIC tbb)
target_link_libraries(d07 PUBLIC tbb)
target_link_libraries(d08 PUBLIC tbb)

install(TARGETS d01 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d02 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d03 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d04 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d05 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d06 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d07 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
install(TARGETS d08 DESTINATION "${CMAKE_SOURCE_DIR}/bin")