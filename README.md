# prometheus-http-query

A rust library that provides an interface to query a Prometheus server and offers the tools to gradually build queries before actually sending them by leveraging Rust's type system. Several kinds of metadata queries are also supported (e.g. retrieving time series, rules, alerts etc.). Check [docs.rs](https://docs.rs/prometheus-http-query) for detailed information.

## Example

```rust
use prometheus_http_query::{Aggregate, Client, Error, InstantVector, RangeVector, Selector};
use prometheus_http_query::aggregations::{sum, topk};
use prometheus_http_query::functions::rate;
use std::convert::TryInto;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let client = Client::default();

    // Construct an instant vector, execute a query, interpret the result
    // again as an instant vector.
    let vector: InstantVector = Selector::new()
        .metric("prometheus_http_requests_total")
        .try_into()?;

    let q = topk(vector, Some(Aggregate::By(&["code"])), 5);

    let response = client.query(q, None, None).await?;

    assert!(response.as_instant().is_some());

    // Construct a range vector, execute a query, interpret the result
    // as an instant vector.
    let vector: RangeVector = Selector::new()
        .metric("node_cpu_seconds_total")?
        .with("mode", "user")
        .range("5m")?
        .try_into()?;
	
    let q = sum(rate(vector), Some(Aggregate::By(&["cpu"])));
    
    let response = client.query(q, None, None).await?;
    
    assert!(response.as_instant().is_some());
    
    // It is also possible to bypass every kind of validation by supplying
    // a custom query directly to the InstantVector | RangeVector types.
    // The equivalent of the operation above would be:
    let q = r#"sum by(cpu) (rate(node_cpu_seconds_total{mode="user"}[5m]))"#;
    
    let v = RangeVector(q.to_string());
    
    let response = client.query(v, None, None).await?;
    
    assert!(response.as_instant().is_some());
   
    Ok(())
}
```

## Compatibility

The table below depicts the compatibility of this library with various versions of the Prometheus server.

| Features | v2.33 | v2.32 | v2.31 | v2.30 | v2.29 |
|---|---|---|---|---|---|
| [Instant queries](https://prometheus.io/docs/prometheus/latest/querying/api/#instant-queries) | yes | yes | yes | yes | yes |
| [Range queries](https://prometheus.io/docs/prometheus/latest/querying/api/#range-queries) | yes | yes | yes | yes | yes |
| [Time series metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#finding-series-by-label-matchers) | yes | yes | yes | yes | yes |
| [Label name metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#getting-label-names) | yes | yes | yes | yes | yes |
| [Label value metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-label-values) | yes | yes | yes | yes | yes |
| [Targets](https://prometheus.io/docs/prometheus/latest/querying/api/#targets) | yes | yes | yes | yes | yes |
| [Rules](https://prometheus.io/docs/prometheus/latest/querying/api/#rules) | yes | yes | yes | yes | yes |
| [Alerts](https://prometheus.io/docs/prometheus/latest/querying/api/#alerts) | yes | yes | yes | yes | yes |
| [Alertmanagers](https://prometheus.io/docs/prometheus/latest/querying/api/#alertmanagers) | yes | yes | yes | yes | yes |
| [Runtime flags](https://prometheus.io/docs/prometheus/latest/querying/api/#flags) | yes | yes | yes | yes | yes |
| [Metric metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-metric-metadata) | yes | yes | yes | yes | yes |
| [Target metadata](https://prometheus.io/docs/prometheus/latest/querying/api/#querying-target-metadata) | yes | yes | yes | yes | no |
| [Trigonometric functions](https://prometheus.io/docs/prometheus/latest/querying/functions/#trigonometric-functions) | yes | yes | yes | no | no |

Server versions below v2.29 are untested.

## Tests

In order to run all tests a Prometheus server must be running at `http://localhost:9090`. No special configuration is required at this point.

Run: `RUSTFLAGS="--cfg unsound_local_offset" cargo test`

## Contributing

Please do not hesitate to file issues in order to report bugs, ask questions or make suggestions. You are also welcome to tackle open issues if there are any.

If you are looking to submit code, please make sure that the tests pass successfully.


