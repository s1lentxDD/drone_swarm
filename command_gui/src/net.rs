//! 成员6待实现：保持 spawn 签名，后续直接替换此文件。
use tokio::sync::mpsc;

pub fn spawn(_server: String, _rx: mpsc::Receiver<String>, tx: mpsc::Sender<String>) {
    let msg = shared::ServerMsg::Log {
        text: "网络模块尚未接入（成员6待实现），当前不会连接服务器。".into(),
    };
    let _ = tx.try_send(serde_json::to_string(&msg).expect("serialize stub status"));
}
