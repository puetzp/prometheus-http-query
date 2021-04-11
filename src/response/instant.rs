use super::{ResultType, Status};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InstantQueryResponse {
    pub status: Status,
    pub data: Data,
    #[serde(alias = "errorType")]
    pub error_type: Option<String>,
    pub error: Option<String>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Metric {
    #[serde(rename = "metric")]
    pub labels: HashMap<String, String>,
    pub value: (f64, String),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Data {
    #[serde(alias = "resultType")]
    pub result_type: ResultType,
    pub result: Vec<Metric>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};
    use std::array::IntoIter;
    use std::iter::FromIterator;

    #[test]
    fn test_deserialize_instant_query_response() {
        let r = InstantQueryResponse {
            status: Status::Success,
            data: Data {
                result_type: ResultType::Vector,
                result: vec![Metric {
                    labels: HashMap::<_, _>::from_iter(IntoIter::new([
                        (String::from("instance"), String::from("localhost:9090")),
                        (String::from("__name__"), String::from("up")),
                        (String::from("job"), String::from("prometheus")),
                    ])),
                    value: (1617960600.0, String::from("1")),
                }],
            },
            error_type: None,
            error: None,
            warnings: None,
        };

        assert_de_tokens(
            &r,
            &[
                Token::Struct {
                    name: "InstantQueryResponse",
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
                    name: "Data",
                    len: 2,
                },
                Token::Str("result_type"),
                Token::Enum { name: "ResultType" },
                Token::UnitVariant {
                    name: "ResultType",
                    variant: "Vector",
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
                Token::Str("value"),
                Token::Tuple { len: 2 },
                Token::F64(1617960600.0),
                Token::Str("1"),
                Token::TupleEnd,
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
