@echo off
setlocal EnableExtensions DisableDelayedExpansion
chcp 65001 >nul
cd /d "%~dp0"
if exist "release\command_gui.exe" (
    start "" "release\command_gui.exe" --demo
    exit /b
)
call "scripts\Prepare-Rust.cmd"
if errorlevel 1 goto failed
call cargo run --locked -p command_gui -- --demo
if errorlevel 1 goto failed
exit /b 0
:failed
pause
exit /b 1

