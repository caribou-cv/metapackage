use std::{error, fmt};
use super::Error;

/// The `Never` type
///
/// The `Never` type is an uninhabited type and represents the type of a value that can never 
/// exist. The never type, `!`, will be used in the near future (issue: tracking issue for 
/// promoting `!` to a type [RFC 1216]).
#[derive(Debug)]
pub enum Never { }

impl fmt::Display for Never {

    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        
        unreachable!()
    }
}

impl error::Error for Never {

    fn description(&self) -> &str {

        unreachable!()
    }
}

impl From<Never> for Error {

	fn from(_: Never) -> Error {

		unreachable!()
	}
}