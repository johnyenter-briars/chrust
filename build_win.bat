@echo off

SET VERSION=0.1.0
SET TARGET_DIR=%CD%\release\win\chrust-%VERSION%\
SET BUILD_TARGET=x86_64-pc-windows-msvc
cargo build --target %BUILD_TARGET% --release
mkdir %TARGET_DIR%
COPY  %CD%\target\%BUILD_TARGET%\release\chrust.exe %TARGET_DIR%
mkdir %TARGET_DIR% 
ROBOCOPY %CD%\boards %TARGET_DIR%\boards /E
ROBOCOPY %CD%\static %TARGET_DIR%\static /E
