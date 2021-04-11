use super::{ResultType, Status};
use serde::de::Deserializer;
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

#[derive(Debug, PartialEq)]
pub struct Metric {
    pub labels: HashMap<String, String>,
    pub epoch: f64,
    pub value: String,
}

impl<'de> Deserialize<'de> for Metric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TmpMetric {
            metric: HashMap<String, String>,
            value: TmpValue,
        }

        #[derive(Deserialize)]
        struct TmpValue {
            epoch: f64,
            value: String,
        }

        let tmp: TmpMetric = Deserialize::deserialize(deserializer)?;

        Ok(Metric {
            labels: tmp.metric,
            epoch: tmp.value.epoch,
            value: tmp.value.value,
        })
    }
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
                    epoch: 1617960600.0,
                    value: String::from("1"),
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
                    name: "TmpMetric",
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
                Token::Struct {
                    name: "TmpValue",
                    len: 2,
                },
                Token::Str("epoch"),
                Token::F64(1617960600.0),
                Token::Str("value"),
                Token::Str("1"),
                Token::StructEnd,
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
