use super::{ResultType, Status};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RangeQueryResponse {
    pub status: Status,
    pub data: Option<Data>,
    #[serde(alias = "errorType")]
    pub error_type: Option<String>,
    pub error: Option<String>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Value {
    pub metric: HashMap<String, String>,
    pub values: Vec<(f64, String)>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Data {
    #[serde(alias = "resultType")]
    pub result_type: ResultType,
    pub result: Vec<Value>,
}
