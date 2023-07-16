use anyhow::Error as AnyhowError;
use anyhow::Result as AnyhowResult;

pub type Result<'a, T> = AnyhowResult<T>;

pub type Error = AnyhowError;
