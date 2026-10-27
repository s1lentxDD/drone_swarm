@echo off
chcp 65001 >nul
cd /d "%~dp0"
if exist ".local\dev-env.cmd" call ".local\dev-env.cmd"
echo Drone Swarm development terminal
echo Try: cargo build --workspace --locked
echo Try: cargo run --locked -p command_gui -- --demo
cmd /k

