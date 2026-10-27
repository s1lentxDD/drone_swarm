# 无人机指挥站（宁波）——成员5交付

依据 [DeepSeek 分享教程](https://chat.deepseek.com/share/t8pfy2di0rgih1b7po) 完成阶段一的工作区搭建与阶段六的 GUI 渲染。

## 先打开什么

Windows 用户在本文件所在目录操作：

1. 双击 `Preview-GUI.cmd`：打开界面演示，显示 d1、d2 两架静态示例无人机。
2. 点击左侧 d1 或 d2，对应地图标记变黄，右侧显示状态。
3. 点击“起飞”“降落”或地图上的位置，右侧日志显示生成的指令。**演示模式只检查界面，不发送网络指令，也不模拟飞行。**
4. 双击 `Start-GUI.cmd`：打开普通模式。成员6的网络实现尚未接入时，列表为空是预期行为。
5. 双击 `Open-Dev-Terminal.cmd`：打开已配置本机 Rust、Git 的命令窗口。输入下列命令后按回车。

```text
cargo build --workspace --locked
cargo test --workspace --locked
cargo run --locked -p command_gui -- --demo
```

如果没有预编译程序，两个启动脚本会通过 Cargo 编译后启动。首次编译需要网络下载依赖。启动失败时窗口会保留错误信息。

## 交付范围

| 目录/文件 | 当前状态 | 后续负责人 |
| --- | --- | --- |
| 根 `Cargo.toml`、`.gitignore`、`Cargo.lock` | 四个 crate 的工作区，锁定依赖版本 | 全组 |
| `shared/` | 加入教程中的协议及坐标函数，供 GUI 编译；需成员1确认 | 成员1 |
| `drone_sim/` | 阶段一占位工程，未实现模拟器 | 成员2 |
| `command_server/` | 阶段一占位工程，未实现服务器 | 成员3、4 |
| `command_gui/src/main.rs` | GUI 入口、命令行参数、窗口尺寸 | 成员5 |
| `command_gui/src/app.rs` | 无人机列表、状态、选中、按钮、日志、消息处理 | 成员5 |
| `command_gui/src/map.rs` | 网格、经纬度映射、标记高亮、点击生成 Goto | 成员5 |
| `command_gui/src/fonts.rs` | 从系统加载中文字体 | 成员5 |
| `command_gui/src/net.rs` | 临时占位，明确提示未连接，不连接服务器 | 成员6覆盖 |
| `command_gui/src/video.rs` | 临时占位面板 | 成员6覆盖 |

## 成员6接入方式

保持以下函数签名，直接替换 `net.rs`、`video.rs`：

```rust
pub fn spawn(server: String, rx: tokio::sync::mpsc::Receiver<String>, tx: tokio::sync::mpsc::Sender<String>);
pub fn render(ui: &mut egui::Ui, drones: &std::collections::HashMap<String, shared::DroneState>, selected: Option<&str>);
```

- `rx` 收到 GUI 生成的 JSON 字符串，不带行尾换行；网络模块发送 TCP 时追加 `\n`。
- `tx` 向 GUI 传递 `shared::ServerMsg` 的 JSON 字符串。
- 普通模式中 GUI 只把指令交给网络队列，不把“入队成功”当作无人机执行成功。
- 接入后使用 `cargo run --locked -p command_gui -- --server 127.0.0.1:9000`，不要带 `--demo`。
- 测试服务器、无人机进程、真实状态广播与视频面板，需要其他成员的模块完成后再联调。

## 对原教程的必要修正

- 在仓库根目录实际创建 `Cargo.toml`；四次 `cargo new` 不会自动创建工作区文件。
- 固定 `eframe`、`egui` 为 `0.27.2`，创建闭包直接返回 `Box<dyn App>`。
- `main` 返回 `eframe::Result<()>`，避免 `anyhow` 与 GUI 错误类型转换不兼容。
- 补充中文字体、空列表提示、发送失败日志、掉线后清理选中项。
- GUI 演示功能与真实网络入口分离；演示不会改变模拟器或其他成员的接口。
- 坐标测试考虑 f32 归一化的舍入误差，并检查边界方向。

## 开发环境

本机通过 `.local/dev-env.cmd` 复用已存在的 Rust GNU 工具链；该文件仅供本机使用，不提交 GitHub。其他电脑请自行安装 Git 和 Rust，然后运行 Cargo 命令。

- Rust 安装：[官方安装页](https://rust-lang.org/tools/install/)
- Windows 默认 MSVC 工具链需要 Visual Studio C++ Build Tools。
- 中文字体从本机系统读取，不随项目分发；也可以设置 `DRONE_GUI_FONT` 为本机字体路径。
- 编译产物在 `target/` 或本机配置的目录，预编译 GUI 在 `release/`（如交付包附带），均不提交仓库。

## 小组协作

仓库建立后，仓库所有者在 GitHub 的 Settings → Collaborators 中邀请另外五位成员。其他成员接受邀请后克隆仓库。成员5后续建议在独立分支修改自己的文件，避免直接覆盖队友的工作。

提交前运行：

```text
cargo fmt --all -- --check
cargo test --workspace --locked
git status
```

成员1应先核对 `shared`；成员6接入前，网络与视频为占位实现。本次交付不等同于全系统联调完成。

