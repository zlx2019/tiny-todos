use std::sync::{atomic::AtomicU64, Arc};

/// 全局状态
#[derive(Debug, Default, Clone)]
pub struct AppState {
    // 请求数统计
    pub counter: Arc<AtomicU64>,
}
