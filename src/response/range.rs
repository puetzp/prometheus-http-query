use super::Status;
use serde::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RangeQueryResponse {
    pub status: Status,
    pub data: Vec<Metric>,
    pub error_type: Option<String>,
    pub error: Option<String>,
    pub warnings: Option<Vec<String>>,
}

impl RangeQueryResponse {
    pub fn is_success(&self) -> bool {
        matches!(self.status, Status::Success)
    }

    pub fn is_error(&self) -> bool {
        matches!(self.status, Status::Error)
    }
}

impl<'de> Deserialize<'de> for RangeQueryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct TmpRangeQueryResponse {
            status: Status,
            data: TmpData,
            #[serde(alias = "errorType")]
            error_type: Option<String>,
            error: Option<String>,
            warnings: Option<Vec<String>>,
        }

        #[derive(Deserialize)]
        struct TmpData {
            result: Vec<Metric>,
        }

        let tmp: TmpRangeQueryResponse = Deserialize::deserialize(deserializer)?;

        Ok(RangeQueryResponse {
            status: tmp.status,
            data: tmp.data.result,
            error_type: tmp.error_type,
            error: tmp.error,
            warnings: tmp.warnings,
        })
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Metric {
    #[serde(rename = "metric")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "values")]
    pub samples: Vec<Value>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Value {
    pub epoch: f64,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};
    use std::array::IntoIter;
    use std::iter::FromIterator;

    #[test]
    fn test_deserialize_range_query_response() {
        let r = RangeQueryResponse {
            status: Status::Success,
            data: vec![Metric {
                labels: HashMap::<_, _>::from_iter(IntoIter::new([
                    (String::from("instance"), String::from("localhost:9090")),
                    (String::from("__name__"), String::from("up")),
                    (String::from("job"), String::from("prometheus")),
                ])),
                samples: vec![
                    Value {
                        epoch: 1617960600.0,
                        value: String::from("1"),
                    },
                    Value {
                        epoch: 1617960900.0,
                        value: String::from("1"),
                    },
                    Value {
                        epoch: 1617961200.0,
                        value: String::from("1"),
                    },
                    Value {
                        epoch: 1617961500.0,
                        value: String::from("1"),
                    },
                ],
            }],
            error_type: None,
            error: None,
            warnings: None,
        };

        assert_de_tokens(
            &r,
            &[
                Token::Struct {
                    name: "TmpRangeQueryResponse",
                    len: 2,
                },
                Token::Str("status"),
                Token::Enum { name: "Status" },
                Token::UnitVariant {
                    name: "Status",
                    variant: "Success",
                },
                Token::Str("data"),
                Token::Struct {
                    name: "TmpData",
                    len: 2,
                },
                Token::Str("result"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Metric",
                    len: 2,
                },
                Token::Str("metric"),
                Token::Map { len: Some(3) },
                Token::Str("instance"),
                Token::Str("localhost:9090"),
                Token::Str("__name__"),
                Token::Str("up"),
                Token::Str("job"),
                Token::Str("prometheus"),
                Token::MapEnd,
                Token::Str("values"),
                Token::Seq { len: Some(4) },
                Token::Struct {
                    name: "Value",
                    len: 2,
                },
                Token::Str("epoch"),
                Token::F64(1617960600.0),
                Token::Str("value"),
                Token::Str("1"),
                Token::StructEnd,
                Token::Struct {
                    name: "Value",
                    len: 2,
                },
                Token::Str("epoch"),
                Token::F64(1617960900.0),
                Token::Str("value"),
                Token::Str("1"),
                Token::StructEnd,
                Token::Struct {
                    name: "Value",
                    len: 2,
                },
                Token::Str("epoch"),
                Token::F64(1617961200.0),
                Token::Str("value"),
                Token::Str("1"),
                Token::StructEnd,
                Token::Struct {
                    name: "Value",
                    len: 2,
                },
                Token::Str("epoch"),
                Token::F64(1617961500.0),
                Token::Str("value"),
                Token::Str("1"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("error_type"),
                Token::None,
                Token::Str("error"),
                Token::None,
                Token::Str("warnings"),
                Token::None,
                Token::StructEnd,
            ],
        )
    }
}
