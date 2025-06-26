use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct CommonError{}

impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for CommonError {}