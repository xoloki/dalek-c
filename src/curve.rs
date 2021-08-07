use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use std::error::Error;
use std::fmt;

// string constants
pub const FOO: &str = "foo";

// the set of errors
#[derive(Debug)]
pub enum CurveError {
    Foo,
}

impl fmt::Display for CurveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CurveError {
    fn description(&self) -> &str {
        match self {
            CurveError::Foo => "Foo",
        }
    }
}
