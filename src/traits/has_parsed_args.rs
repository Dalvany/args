use std::str::FromStr;

use super::super::{Args,ArgsError};
use super::super::validations::Validation;

/// A trait designed to bind parsed arguments and instance methods to a struct.
pub trait HasParsedArgs: Send {
    // Instance methods
    /// Returns a references to the parsed `Args` struct.
    fn parsed_args(&self) -> &Args;

    // Defaulted instance methods
    /// Acts as a convenience method for calling the `Args` implementation.
    fn has_value(&self, opt_name: &str) -> bool {
        self.parsed_args().has_value(opt_name)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn validated_value_of<T: FromStr>(&self, opt_name: &str, validations: &[Box<Validation<T=T>>]) -> Result<T, ArgsError> {
        self.parsed_args().validated_value_of::<T>(opt_name, validations)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn value_of<T: FromStr>(&self, opt_name: &str) -> Result<T, ArgsError> {
        self.parsed_args().value_of::<T>(opt_name)
    }
}
