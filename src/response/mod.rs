pub mod instant;
pub mod range;
use serde::Deserialize;

pub trait QueryResponse {}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Status {
    #[serde(alias = "success")]
    Success,
    #[serde(alias = "error")]
    Error,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ResultType {
    #[serde(alias = "matrix")]
    Matrix,
    #[serde(alias = "vector")]
    Vector,
    #[serde(alias = "scalar")]
    Scalar,
    #[serde(alias = "string")]
    String,
}
