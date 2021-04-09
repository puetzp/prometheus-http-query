use serde::Deserialize;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Status {
    #[serde(alias = "success")]
    Success,
    #[serde(alias = "error")]
    Error,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Value {
    pub timestamp: f64,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Metric {
    #[serde(rename = "metric")]
    pub labels: HashMap<String, String>,
    pub value: Value,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Data {
    #[serde(alias = "resultType")]
    pub result_type: ResultType,
    pub result: Vec<Metric>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct QueryResult {
    pub status: Status,
    pub data: Option<Data>,
    #[serde(alias = "errorType")]
    pub error_type: Option<String>,
    pub error: Option<String>,
    pub warnings: Option<Vec<String>>,
}
