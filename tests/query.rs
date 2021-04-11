use prometheus_http_query::{Client, InstantQuery, RangeQuery};

#[test]
fn test_instant_query() {
    let client: Client = Default::default();
    let query = InstantQuery {
        query: "up",
        time: None,
        timeout: None,
    };
    let result = tokio_test::block_on(async { client.instant(&query).await.unwrap() });
    assert!(!result.data.result.is_empty());
}

#[test]
fn test_range_query() {
    let client: Client = Default::default();
    let query = RangeQuery {
        query: "up{job=\"prometheus\"}",
        start: "2021-04-09T11:30:00.000+02:00",
        end: "2021-04-09T12:30:00.000+02:00",
        step: "5m",
        timeout: None,
    };
    let result = tokio_test::block_on(async { client.range(&query).await.unwrap() });
    assert!(!result.data.result.is_empty());
}
