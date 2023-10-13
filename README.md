# prometheus-http-query

This crate provides an interface to the [Prometheus HTTP API](https://prometheus.io/docs/prometheus/latest/querying/api/) and leverage Rust's type system in the process where applicable.

## Example

```rust
use prometheus_http_query::{Client, Error, Selector, RuleKind};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let client = Client::default();

    // Evaluate a PromQL query.
    let q = "topk by (code) (5, prometheus_http_requests_total)";
    let response = client.query(q).get().await?;
    assert!(response.data().as_vector().is_some());

    // Retrieve active alerts.
    let alerts = client.alerts().await;
    assert!(alerts.is_ok());

    // Retrieve recording rules.
    let recording_rules = client.rules().kind(RuleKind::Recording).get().await;
    assert!(recording_rules.is_ok());

    // Retrieve a list of time series that match certain labels sets ("series selectors").
    let select1 = Selector::new()
        .eq("handler", "/api/v1/query");

    let select2 = Selector::new()
        .eq("job", "node")
        .regex_eq("mode", ".+");

    let time_series = client.series(&[select1, select2]).get().await;
    assert!(time_series.is_ok());

    Ok(())
}
```

## Compatibility

This library is generally compatible with Prometheus versions starting from v2.30. Individual client methods might fail with older versions as newer versions of Prometheus server support additional methods and query parameters. Run Prometheus server version >= 2.46 to ensure maximum compatibility.

## Tests

In order to run all tests a Prometheus server must be running at `http://localhost:9090`. No special configuration is required at this point, simply run: `cargo test`

## Contributing

Please do not hesitate to file issues in order to report bugs, ask questions or make suggestions. You are also welcome to tackle open issues if there are any.

If you are looking to submit code, please make sure that the tests pass successfully.


