@echo off
rem Called inside the launcher's SETLOCAL so environment changes stay local.
if exist "%~dp0..\.local\dev-env.cmd" call "%~dp0..\.local\dev-env.cmd"
where.exe cargo >nul 2>nul
if not errorlevel 1 exit /b 0
if defined CARGO_HOME if exist "%CARGO_HOME%\bin\cargo.exe" set "PATH=%CARGO_HOME%\bin;%PATH%"
where.exe cargo >nul 2>nul
if not errorlevel 1 exit /b 0
if exist "%USERPROFILE%\.cargo\bin\cargo.exe" set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
where.exe cargo >nul 2>nul
if not errorlevel 1 exit /b 0
echo.
echo [未找到 Cargo] 当前下载包是源码，运行前需要 Rust 开发环境。
echo.
echo 只想打开界面：请从下方 Releases 下载 Windows x64 运行包，完整解压后双击 Preview-GUI.cmd。
echo https://github.com/s1lentxDD/drone_swarm/releases/latest
echo 不要选择 Source code 或 Code 菜单下的 Download ZIP；它们不包含预编译程序。
echo.
echo 需要修改和编译源码：请按 Rust 官方说明安装，然后重新打开命令窗口。
echo https://rust-lang.org/tools/install/
echo 可运行 cargo --version 检查安装结果。
exit /b 1
