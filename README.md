# prometheus-http-query

A rust library that provides an interface to query a Prometheus server and offers the tools to gradually build queries before actually sending them by leveraging Rust's type system. Several kinds of metadata queries are also supported (e.g. retrieving time series, rules, alerts etc.). Check [docs.rs](https://docs.rs/prometheus-http-query) for detailed information.

Tested with Prometheus v2.31.

## Example

```rust
use prometheus_http_query::{Aggregate, Client, Error, InstantVector, RangeVector, Selector};
use prometheus_http_query::aggregations::{sum, topk};
use prometheus_http_query::functions::rate;
use std::convert::TryInto;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let client = Client::default();
    
    let vector: InstantVector = Selector::new()
        .metric("prometheus_http_requests_total")
        .try_into()?;

    let q = topk(vector, Some(Aggregate::By(&["code"])), 5);

    let response = client.query(q, None, None).await?;

    assert!(response.as_instant().is_some());

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