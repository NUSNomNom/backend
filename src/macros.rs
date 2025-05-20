#[macro_export]
macro_rules! error_ctx {
    ($($arg:tt)*) => {
        || {
            let msg = format!($($arg)*);
            tracing::error!(msg);
            msg
        }
    };
}
