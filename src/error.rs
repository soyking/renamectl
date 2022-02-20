use std::{fmt, result};

pub type Result<'a, T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    original_message: String,
    message: String,
}

impl Error {
    pub fn new(original_message: String, message: String) -> Error {
        return Error {
            original_message,
            message,
        };
    }

    pub fn from_string(original_stringer: &dyn ToString, message: &str) -> Error {
        return Error::new(original_stringer.to_string(), message.to_string());
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, original error: {}",
            self.message, self.original_message
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    #[test]
    fn test_error_new() {
        let e1 = "test1".to_string();
        let e1_clone = e1.clone();
        let e2 = "test2".to_string();
        let e2_clone = e2.clone();

        let e = Error::new(e1, e2);
        println!("err message: {:?}", &e);
        assert_eq!(e2_clone + ", original error: " + &e1_clone, e.to_string());
    }

    #[test]
    fn test_error_from_string() {
        use std::io::{Error as IOError, ErrorKind};

        let not_found = ErrorKind::NotFound;
        let e1 = IOError::from(not_found);

        let e2 = "test1";

        let e = Error::from_string(&e1, &e2);
        println!("err message: {:?}", &e);
        assert_eq!(
            e2.to_string() + ", original error: entity not found",
            e.to_string()
        );
    }
}
