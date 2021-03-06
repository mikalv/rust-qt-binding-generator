cmake_minimum_required(VERSION 3.0 FATAL_ERROR)

project (rust_qt_binding_generator)

set(QT_MIN_VERSION "5.6.0")
set(KF5_MIN_VERSION "5.2.0")

LIST(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(ECM 1.0.0 NO_MODULE)
if (ECM_FOUND)
    LIST(APPEND CMAKE_MODULE_PATH ${ECM_MODULE_PATH} ${ECM_KDE_MODULE_DIR})
    include(KDEInstallDirs)
    include(KDECMakeSettings)
    include(KDECompilerSettings NO_POLICY_SCOPE)
    include(ECMPoQmTools)
else()
    if (POLICY CMP0063)
        cmake_policy(SET CMP0063 NEW)
    endif()
    enable_testing()
    set(CMAKE_AUTOMOC ON)
endif()

set(CMAKE_AUTOUIC ON)

include(FeatureSummary)

find_package(Cargo REQUIRED)
find_package(Rust REQUIRED)
set(CMAKE_THREAD_PREFER_PTHREAD TRUE)
find_package(Threads REQUIRED)

# Runtime information library -- dl on Debian, execinfo on FreeBSD.
# This quiets a warning from rustc about linking to native artifacts.
if (CMAKE_SYSTEM MATCHES "FreeBSD")
    find_library(DL_LIBRARY execinfo)
else()
    find_library(DL_LIBRARY ${CMAKE_DL_LIBS})
endif()

# It would be neater to use set_package_properties() here and to
# rely on feature_summary(), below, but that entails using find_package()
# and moving this whole thing into a Find-module, which is over-wrought.
if (NOT DL_LIBRARY)
    message(FATAL_ERROR "No runtime information library (-ldl or -lexecinfo)")
endif()

# Find Qt modules
find_package(Qt5 ${QT_MIN_VERSION} CONFIG
    REQUIRED COMPONENTS
        Core
        Test
    OPTIONAL_COMPONENTS
        Widgets
        Svg
        Quick
)
find_package(Qt5QuickControls2 EXACT ${Qt5Core_VERSION})
find_package(Qt5Charts EXACT ${Qt5Core_VERSION})

find_package(KF5Kirigami2)

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)

string(TOUPPER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_UPPER)
if(CMAKE_BUILD_TYPE_UPPER STREQUAL DEBUG)
    set(RUST_TARGET_DIR target/debug/)
    set(RUST_BUILD_FLAG)
else()
    set(RUST_TARGET_DIR target/release/)
    set(RUST_BUILD_FLAG --release)
endif()

add_subdirectory(src)

add_subdirectory(tests)

if(Qt5Widgets_FOUND AND Qt5Svg_FOUND)
    add_subdirectory(demo)
endif()

if (ECM_FOUND)
    install(FILES org.kde.rust_qt_binding_generator.appdata.xml
            DESTINATION ${KDE_INSTALL_METAINFODIR})
endif()
