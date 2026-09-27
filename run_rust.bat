@echo off
title AirMesh Native Rust Daemon
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

echo =========================================================
echo   Compiling and Running AirMesh Native Rust Core...
echo =========================================================
echo.

start "" http://localhost:47823
cargo run -p airmesh-daemon
pause
