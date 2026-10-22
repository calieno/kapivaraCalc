# ============================================================================
# CMakeLists.txt - Reconstructed HP 49G Emulator (YUser.exe)
# Reverse Engineered with REA (Reverse Engineer Anything)
# Original Author: Sebastien Carlier / Hewlett-Packard (1999)
# ============================================================================
cmake_minimum_required(VERSION 3.10)
project(HP49_YUser_Reconstructed CXX C)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Definitions
add_definitions(-DUNICODE -D_UNICODE -DWIN32_LEAN_AND_MEAN)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Source files
set(SOURCES
    src/YUser.cpp
)

# Header files
set(HEADERS
    include/YUser.h
    include/YorkeM.h
    include/YMHelper.h
)

if(WIN32)
    # Target executable for Windows
    add_executable(YUser WIN32 ${SOURCES} ${HEADERS})

    # Link Windows standard libraries
    target_link_libraries(YUser
        user32
        gdi32
        shell32
        comctl32
    )

    # Note: YorkeM.dll and YMHelper.dll are dynamically loaded via LoadLibrary
    # or linked against their import libraries if available.
    message(STATUS "Configured YUser for Windows build.")
else()
    # Cross-compilation with MinGW or stub build for Linux analysis
    message(STATUS "Configured YUser build. If cross-compiling, use MinGW (x86_64-w64-mingw32-g++ or i686-w64-mingw32-g++).")
    add_library(YUser_Reconstructed_Static STATIC ${SOURCES} ${HEADERS})
endif()
