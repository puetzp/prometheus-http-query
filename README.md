# prometheus-http-query

A rust library that provides an interface to query a Prometheus server and offers the tools to gradually build queries before actually sending them by leveraging Rust's type system. Check [docs.rs](https://docs.rs/prometheus-http-query) for detailed information.

## Example

```rust
use prometheus_http_query::{Client, Selector, RangeVector, Aggregate};
use prometheus_http_query::aggregations::sum;
use prometheus_http_query::functions::rate;
use std::convert::TryInto;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), prometheus_http_query::Error> {
    let client: Client = Default::default();
    
    let v: RangeVector = Selector::new()
        .metric("node_cpu_seconds_total")?
        .with("mode", "user")
        .range("5m")?
        .try_into()?;
	
    let q = sum(rate(v), Some(Aggregate::By(&["cpu"])));
    
    let response = client.query(q, None, None).await;
    
    assert!(response.is_ok());
    
    // It is also possible to bypass every kind of validation by supplying
    // a custom query directly to the InstantVector | RangeVector types.
    // The equivalent of the operation above would be:
    let q = r#"sum by(cpu) (rate(node_cpu_seconds_total{mode="user"}[5m]))"#;
    
    let v = RangeVector(q.to_string());
    
    let response = client.query(v, None, None).await;
    
    assert!(response.is_ok());
   
    Ok(())
}
```