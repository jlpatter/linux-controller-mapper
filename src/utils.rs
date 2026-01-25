use std::sync::PoisonError;
use anyhow::anyhow;

pub fn lock_error_handler<T>(err: PoisonError<T>) -> anyhow::Error {
    anyhow!("ERROR: Mutex poisoning occurred! This really bad :( see details: {err}")
}

pub fn lock_error_handler_string<T>(err: PoisonError<T>) -> String {
    lock_error_handler(err).to_string()
}
