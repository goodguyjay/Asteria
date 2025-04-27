add_library(Argus
        Dummy.cpp
)

target_include_directories(Argus PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)