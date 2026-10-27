@echo off
setlocal EnableExtensions DisableDelayedExpansion
chcp 65001 >nul
cd /d "%~dp0"
call "scripts\Prepare-Rust.cmd"
if errorlevel 1 goto failed
echo Drone Swarm development terminal
echo Try: cargo build --workspace --locked
echo Try: cargo run --locked -p command_gui -- --demo
cmd /k
exit /b
:failed
pause
exit /b 1

