@echo off
title AirMesh - 1-Click Launcher
echo =========================================================
echo   AirMesh Local Peer-to-Peer Mesh (1-Click Start)
echo =========================================================
echo.

:: Automatically open browser after 2 seconds
start "" http://localhost:47823

:: Start the server
node dev_server.cjs
pause
