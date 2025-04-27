add_library(Communication
        Dummy.cpp
)

target_include_directories(Communication PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)