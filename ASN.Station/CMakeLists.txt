cmake_minimum_required(VERSION 3.16)
project(ASN-Station LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(spdlog REQUIRED)

add_subdirectory(Argus)
add_subdirectory(Core)
add_subdirectory(Common)
add_subdirectory(Communication)
add_subdirectory(Database)

add_executable(ASN-Station main.cpp)

target_link_libraries(ASN-Station PRIVATE
        Argus
        Core
        Common
        Communication
        Database
        spdlog::spdlog
)