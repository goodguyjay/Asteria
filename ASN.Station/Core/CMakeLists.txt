add_library(Core
        Dummy.cpp
)

target_include_directories(Core PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(Core PRIVATE Common)