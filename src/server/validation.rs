//! Validation module

use std::str;

/// Validates data
pub async fn validate(buf: &[u8]) -> anyhow::Result<i64> {
    let s = str::from_utf8(buf)?;
    let num = s.trim().parse::<i64>()?;

    Ok(num)
}
