use std::{error, result};

pub type Error = Box<dyn error::Error + Send + Sync>;

pub type Result<T> = result::Result<T, Error>;
