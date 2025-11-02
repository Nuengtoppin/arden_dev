//! Butler — logging facade (stub)
pub fn info(msg: &str) { tracing::info!(target: "butler", "{}", msg); }
pub fn warn(msg: &str) { tracing::warn!(target: "butler", "{}", msg); }
pub fn error(msg:&str) { tracing::error!(target: "butler", "{}", msg); }