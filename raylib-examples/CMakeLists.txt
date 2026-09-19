cmake_minimum_required(VERSION 3.29)

project(ray_examples VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# ==============================================
# compiler options
# ==============================================
if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    message("using GCC version: ${CMAKE_CXX_COMPILER_VERSION}")
    if (CMAKE_CXX_COMPILER_VERSION VERSION_LESS 13.3)
        message(FATAL_ERROR "Error: GCC version must be 13.3 or higher")
    endif ()
endif ()

if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    message("using Clang version: ${CMAKE_CXX_COMPILER_VERSION}")
    if (CMAKE_CXX_COMPILER_VERSION VERSION_LESS 18.0)
        message(FATAL_ERROR "Error: Clang version must be 18.0 or higher")
    endif ()
endif ()

if (MSVC)
    message("using MSVC version: ${MSVC_VERSION}")
    if (MSVC_VERSION LESS 1944)
        message(FATAL_ERROR "Error: MSVC version must be 19.44 or higher")
    endif ()
endif ()

# ==============================================
# raylib and raylib-cpp
#
# raylib-6.0_win64_mingw-w64
# raylib-6.0_linux_amd64
# ==============================================
# raylib
if (WIN32)
    set(RAYLIB_DIR
            "${CMAKE_CURRENT_SOURCE_DIR}/raylib-6.0_win64_mingw-w64"
    )
else()
    set(RAYLIB_DIR
            "${CMAKE_CURRENT_SOURCE_DIR}/raylib-6.0_linux_amd64"
    )
endif ()

# raylib-cpp
set(RAYLIB_CPP_DIR
        "${CMAKE_CURRENT_SOURCE_DIR}/raylib-cpp"
)

# ==============================================
# copy libs
# ==============================================
function(copy_mingw_runtime TARGET)
    if (WIN32 AND CMAKE_BUILD_TYPE STREQUAL "Debug")
        get_filename_component(MINGW_BIN "${CMAKE_CXX_COMPILER}" DIRECTORY)

        add_custom_command(TARGET ${TARGET} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${MINGW_BIN}/libwinpthread-1.dll"
                "$<TARGET_FILE_DIR:${TARGET}>"

            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${MINGW_BIN}/libgcc_s_seh-1.dll"
                "$<TARGET_FILE_DIR:${TARGET}>"

            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${MINGW_BIN}/libstdc++-6.dll"
                "$<TARGET_FILE_DIR:${TARGET}>"
        )
    else()
        add_custom_command(TARGET ray_state POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${RAYLIB_DIR}/lib/libraylib.so.600"
                "$<TARGET_FILE_DIR:ray_state>/libraylib.so.600"
        )
    endif()
endfunction()

# ==============================================
# ray_state
# ==============================================
add_executable(ray_state
        ray_state/src/main.cpp
        ray_state/src/game/game.cpp
        ray_state/src/states/main_menu.cpp
        ray_state/src/states/playing.cpp
        ray_state/src/states/paused.cpp
        ray_state/src/states/game_over.cpp
)

target_include_directories(
        ray_state
        PRIVATE
        ray_state/src
        ray_state/src/states
        "${RAYLIB_DIR}/include"
        "${RAYLIB_CPP_DIR}/include"
)
if (WIN32)
    target_link_libraries(
            ray_state
            PRIVATE
            "${RAYLIB_DIR}/lib/libraylib.a"
            winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray_state
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray_state
        PRIVATE
        raylib
    )

    set_target_properties(
        ray_state
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray_state)

# ==============================================
# ray-hitbox
# ==============================================

add_executable(ray-hitbox
    ray-hitbox/src/main.cpp
)

target_include_directories(
        ray-hitbox
        PRIVATE
        ray-hitbox/src
        "${RAYLIB_DIR}/include"
)

if (WIN32)
    # static (.a)
    target_link_libraries(
            ray-hitbox
            PRIVATE
            "${RAYLIB_DIR}/lib/libraylib.a"
            winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray-hitbox
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray-hitbox
        PRIVATE
        raylib
    )

    set_target_properties(
        ray-hitbox
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()
copy_mingw_runtime(ray-hitbox)

# ==============================================
# ray-ball
# ==============================================

add_executable(ray-ball
    ray-ball/src/main.cpp
)

target_include_directories(
        ray-ball
        PRIVATE
        ray-ball/src
        "${RAYLIB_DIR}/include"
)

if (WIN32)
    # static (.a)
    target_link_libraries(
            ray-ball
            PRIVATE
            "${RAYLIB_DIR}/lib/libraylib.a"
            winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray-ball
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray-ball
        PRIVATE
        raylib
    )

    set_target_properties(
        ray-ball
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray-ball)

# ==============================================
# ray-render
# ==============================================

add_executable(ray-render
    ray-render/src/main.cpp
)

target_include_directories(
        ray-render
        PRIVATE
        ray-render/src
        "${RAYLIB_DIR}/include"
)

if (WIN32)
    # static (.a)
    target_link_libraries(
            ray-render
            PRIVATE
            "${RAYLIB_DIR}/lib/libraylib.a"
            winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray-render
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray-render
        PRIVATE
        raylib
    )

    set_target_properties(
        ray-render
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray-render)


# ==============================================
# ray-parallax
# ==============================================

add_executable(ray-parallax
    ray-parallax/src/main.cpp
)

target_include_directories(
        ray-parallax
        PRIVATE
        ray-parallax/src
        "${RAYLIB_DIR}/include"
)

if (WIN32)
    # static (.a)
    target_link_libraries(
            ray-parallax
            PRIVATE
            "${RAYLIB_DIR}/lib/libraylib.a"
            winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray-parallax
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray-parallax
        PRIVATE
        raylib
    )

    set_target_properties(
        ray-parallax
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray-parallax)

# ==============================================
# ray_tileset
# ==============================================
add_executable(ray_tileset
        ray_tileset/src/main.cpp
)

target_include_directories(
        ray_tileset
        PRIVATE
        ray_tileset/src
        "${RAYLIB_DIR}/include"
        "${RAYLIB_TILESON}/include"
        "${RAYLIB_TILESON}/src"
)

add_subdirectory(raylib-tileson)

if (WIN32)
    target_link_libraries(
        ray_tileset
        PRIVATE
        "${RAYLIB_DIR}/lib/libraylib.a"
        raylib-tileson
        winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray_tileset
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray_tileset
        PRIVATE
        raylib
    )

    set_target_properties(
        ray_tileset
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray_tileset)

# ==============================================
# ray_mouse
# ==============================================
add_executable(ray_mouse
    ray_mouse/src/main.cpp
)

target_include_directories(
    ray_mouse
    PRIVATE
    ray_mouse/src
    "${RAYLIB_DIR}/include"
)


if (WIN32)
    target_link_libraries(
        ray_mouse
        PRIVATE
        "${RAYLIB_DIR}/lib/libraylib.a"
        winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray_mouse
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray_mouse
        PRIVATE
        raylib
    )

    set_target_properties(
        ray_mouse
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray_mouse)

# ==============================================
# ray_platforms
# ==============================================
add_executable(ray_platforms
    ray_platforms/src/main.cpp
)

target_include_directories(
    ray_platforms
    PRIVATE
    ray_platforms/src
    "${RAYLIB_DIR}/include"
)


if (WIN32)
    target_link_libraries(
        ray_platforms
        PRIVATE
        "${RAYLIB_DIR}/lib/libraylib.a"
        winmm
    )
else()
    # dynamic (.so)
    target_link_directories(
        ray_platforms
        PRIVATE
        "${RAYLIB_DIR}/lib"
    )

    target_link_libraries(
        ray_platforms
        PRIVATE
        raylib
    )

    set_target_properties(
        ray_platforms
        PROPERTIES
        BUILD_RPATH "\$ORIGIN"
        INSTALL_RPATH "\$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
        INSTALL_RPATH_USE_LINK_PATH FALSE
        LINK_FLAGS "-Wl,--disable-new-dtags"
    )
endif()

copy_mingw_runtime(ray_platforms)

# ==============================================
# configure for static
# ==============================================
if (WIN32)
    target_link_options(ray_state PRIVATE
            # -Wl,--verbose
            -static-libgcc
            -static-libstdc++
    )
endif ()

# ==============================================
# Disable the console in Windows releases
# ==============================================
if (WIN32 AND CMAKE_BUILD_TYPE STREQUAL "Release")
    set_target_properties(ray_state PROPERTIES
            WIN32_EXECUTABLE TRUE
    )

    set(CMAKE_EXE_LINKER_FLAGS
            "${CMAKE_EXE_LINKER_FLAGS} -mwindows"
    )

    set(CMAKE_EXE_LINKER_FLAGS
            "${CMAKE_EXE_LINKER_FLAGS} -Wl,-subsystem,windows"
    )
endif ()
