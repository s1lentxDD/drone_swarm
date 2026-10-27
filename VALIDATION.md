# 验证记录

验证日期：2026-10-28（北京时间）。系统：Windows x64；Rust/Cargo 1.99.1；eframe/egui 0.27.2。

## 已通过

- `cargo build --workspace --locked`：四个 crate 编译成功。
- `cargo fmt --all -- --check`：格式检查通过。
- `cargo test --workspace --locked`：共 6 个测试通过，0 失败。
- `cargo build -p command_gui --release --locked`：正式版可执行程序构建成功。
- 实际打开 GUI：中文字体正常，左右面板与地图正常显示。
- 演示模式显示 d1/d2，选择 d1 后地图标记变黄并显示详细状态。
- 起飞、降落分别生成 `takeoff`、`land` 指令；点击地图生成含经纬度的 `goto` 指令。
- 演示模式明确标注“示例数据”“不发送指令”，测试确认演示指令不会进入网络队列。
- 检查 GUI 指令与共享协议的 JSON 兼容性，快照移除无人机时清理选中状态，网络通道关闭时显示发送失败。

## 仍需小组联调

当前 `drone_sim`、`command_server` 是阶段一骨架；GUI 的 `net.rs`、`video.rs` 是阶段六占位模块。未验证真实 TCP 通信、无人机运动、状态广播或视频流。这些属于其他成员实现后的全组联调。

成员1应核对并接管 `shared/`；本项目加入的共享协议来自原教程，仅用于让成员5独立编译。
