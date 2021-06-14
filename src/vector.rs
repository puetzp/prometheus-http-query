use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InstantVector(pub(crate) String);

impl fmt::Display for InstantVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let InstantVector(s) = self;
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub struct RangeVector(pub(crate) String);

impl fmt::Display for RangeVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let RangeVector(s) = self;
        write!(f, "{}", s)
    }
}
