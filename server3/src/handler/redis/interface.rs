use crate::util::global::{GLOBAL};

pub async fn run(_data: &[u8]) -> String {
    if let Ok(slb) = GLOBAL.lock() {
        slb.metrics_tree.handler_call.with_label_values(&["global", "redis"]).inc();
    }
    return "+OK\r\n".to_string();
}
