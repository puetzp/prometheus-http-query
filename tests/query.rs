use prometheus_http_query::aggregations::*;
use prometheus_http_query::functions::*;
use prometheus_http_query::{Aggregate, Client, InstantVector, RangeVector, Selector};
use std::convert::TryInto;

#[test]
fn test_query_1() {
    let client = Client::default();

    let v: InstantVector = Selector::new()
        .metric("node_cpu_seconds_total")
        .unwrap()
        .try_into()
        .unwrap();

    let s = sum(v, Some(Aggregate::By(&["mode"])));

    let response = tokio_test::block_on(async { client.query(s, None, None).await.unwrap() });

    assert!(response.as_instant().is_some());
}

#[test]
fn test_query_2() {
    let client = Client::default();

    let v: RangeVector = Selector::new()
        .metric("node_cpu_seconds_total")
        .unwrap()
        .with("mode", "user")
        .range("5m")
        .unwrap()
        .try_into()
        .unwrap();

    let s = rate(v);

    let response = tokio_test::block_on(async { client.query(s, None, None).await.unwrap() });

    assert!(response.as_instant().is_some());
}

#[test]
fn test_query_3() {
    let client = Client::default();

    let v: RangeVector = Selector::new()
        .metric("node_node_cpu_seconds_total")
        .unwrap()
        .with("mode", "user")
        .range("5m")
        .unwrap()
        .try_into()
        .unwrap();

    let s = rate(v);

    let response = tokio_test::block_on(async {
        client
            .query_range(s, 1623345960, 1623841309, "5m", None)
            .await
            .unwrap()
    });

    assert!(response.as_range().is_some());
}

#[test]
fn test_query_4() {
    let client = Client::default();

    let v: RangeVector = Selector::new()
        .metric("node_node_cpu_seconds_total")
        .unwrap()
        .with("mode", "user")
        .range("5m")
        .unwrap()
        .try_into()
        .unwrap();

    let s = sum(rate(v), Some(Aggregate::By(&["cpu"])));

    let response =
        tokio_test::block_on(async { client.query(s, Some(1623345960), None).await.unwrap() });

    assert!(response.as_instant().is_some());
}

#[test]
fn test_query_5() {
    let client = Client::default();

    let v: RangeVector = Selector::new()
        .metric("node_cpu_seconds_total")
        .unwrap()
        .range("20m")
        .unwrap()
        .try_into()
        .unwrap();

    let s = sum(predict_linear(v, 3600.0), Some(Aggregate::By(&["mode"])));

    let response = tokio_test::block_on(async { client.query(s, None, None).await.unwrap() });

    assert!(response.as_instant().is_some());
}

#[test]
fn test_query_6() {
    let client = Client::default();

    let v: InstantVector = Selector::new()
        .metric("node_cpu_seconds_total")
        .unwrap()
        .try_into()
        .unwrap();

    let s = sum(v, Some(Aggregate::By(&["mode"]))) + 5.0;

    let response = tokio_test::block_on(async { client.query(s, None, None).await.unwrap() });

    assert!(response.as_instant().is_some());
}

#[test]
fn test_query_7() {
    let client = Client::default();

    let v: InstantVector = Selector::new()
        .metric("node_cpu_seconds_total")
        .unwrap()
        .try_into()
        .unwrap();

    let s = round(sum(v, Some(Aggregate::By(&["mode"]))), Some(2.0));

    let response = tokio_test::block_on(async { client.query(s, None, None).await.unwrap() });

    assert!(response.as_instant().is_some());
}
