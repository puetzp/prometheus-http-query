pub mod instant;
pub mod range;
use serde::Deserialize;

pub trait Response {
    fn is_success(&self) -> bool;
    fn is_error(&self) -> bool;
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Status {
    #[serde(alias = "success")]
    Success,
    #[serde(alias = "error")]
    Error,
}
