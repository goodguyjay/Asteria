add_library(Common
        src/Logger.cpp
        include/Common/ToString.hpp
)

target_include_directories(Common PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

find_package(spdlog REQUIRED)
target_link_libraries(Common PUBLIC spdlog::spdlog)
