use anyhow::Error as AnyhowError;
use anyhow::Result as AnyhowResult;

pub type Result<'a, T> = AnyhowResult<T>;

#[allow(dead_code)]
pub type Error = AnyhowError;
