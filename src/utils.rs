use std::sync::PoisonError;
use anyhow::anyhow;

pub fn lock_error_handler<T>(err: PoisonError<T>) -> anyhow::Error {
    anyhow!("ERROR: Mutex poisoning occurred! This really bad :( see details: {err}")
}
