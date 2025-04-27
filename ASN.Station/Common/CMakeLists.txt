add_library(Common
        Logger.cpp
)

target_include_directories(Common PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)

find_package(spdlog REQUIRED)
target_link_libraries(Common PUBLIC spdlog::spdlog)