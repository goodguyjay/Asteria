add_library(Database
        Dummy.cpp
)

target_include_directories(Database PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)