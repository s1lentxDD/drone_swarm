@echo off
chcp 65001 >nul
cd /d "%~dp0"
if exist "release\command_gui.exe" (
    start "" "release\command_gui.exe"
    exit /b
)
if exist ".local\dev-env.cmd" call ".local\dev-env.cmd"
cargo run --locked -p command_gui
if errorlevel 1 pause

