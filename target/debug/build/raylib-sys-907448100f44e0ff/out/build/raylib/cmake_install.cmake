# Install script for directory: /Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/build/raylib/libraylib.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libraylib.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libraylib.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libraylib.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src/raylib.h"
    "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src/rlgl.h"
    "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src/physac.h"
    "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src/raymath.h"
    "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src/raudio.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/build/raylib/raylib.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/build/raylib/raylib-config-version.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/raylib/src/../cmake/raylib-config.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/ralphgreer-walker/Code/voronoi/voronoi/target/debug/build/raylib-sys-907448100f44e0ff/out/build/raylib/external/glfw/cmake_install.cmake")

endif()

