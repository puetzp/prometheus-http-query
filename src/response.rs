//! All types that may be returned as part of return types from [crate::Client] methods.
use crate::util::{AlertState, RuleHealth, TargetHealth};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use time::OffsetDateTime;
use url::Url;

mod de {
    use serde::{Deserialize, Deserializer};
    use std::str::FromStr;
    use time::format_description::well_known::Rfc3339;
    use time::OffsetDateTime;
    use url::Url;

    pub(crate) fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let num = f64::from_str(&raw).map_err(serde::de::Error::custom)?;
        Ok(num)
    }

    pub(crate) fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let url = Url::parse(&raw).map_err(serde::de::Error::custom)?;
        Ok(url)
    }

    pub(crate) fn deserialize_rfc3339<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;

        OffsetDateTime::parse(&raw, &Rfc3339)
            .map_err(|e| serde::de::Error::custom(format!("error parsing '{}': {}", raw, e)))
    }
}

/// A wrapper for possible result types of expression queries ([crate::Client::query] and [crate::Client::query_range]).
#[derive(Debug)]
pub enum QueryResultType {
    Vector(Vec<InstantVector>),
    Matrix(Vec<RangeVector>),
}

impl QueryResultType {
    /// If the result type of the query is `vector`, returns an array of [InstantVector]s. Returns `None` otherwise.
    pub fn as_instant(&self) -> Option<&[InstantVector]> {
        match self {
            QueryResultType::Vector(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the result type of the query is `matrix` returns an array of [RangeVector]s. Returns `None` otherwise.
    pub fn as_range(&self) -> Option<&[RangeVector]> {
        match self {
            QueryResultType::Matrix(v) => Some(v.as_ref()),
            _ => None,
        }
    }
}

/// A single time series containing a single data point/sample.
#[derive(Debug, PartialEq, Deserialize)]
pub struct InstantVector {
    pub(crate) metric: HashMap<String, String>,
    #[serde(alias = "value")]
    pub(crate) sample: Sample,
}

impl InstantVector {
    /// Returns a reference to the set of labels (+ metric name)
    /// of this time series.
    pub fn metric(&self) -> &HashMap<String, String> {
        &self.metric
    }

    /// Returns a reference to the sample of this time series.
    pub fn sample(&self) -> &Sample {
        &self.sample
    }
}

/// A single time series containing a range of data points/samples.
#[derive(Debug, PartialEq, Deserialize)]
pub struct RangeVector {
    pub(crate) metric: HashMap<String, String>,
    #[serde(alias = "values")]
    pub(crate) samples: Vec<Sample>,
}

impl RangeVector {
    /// Returns a reference to the set of labels (+ metric name)
    /// of this time series.
    pub fn metric(&self) -> &HashMap<String, String> {
        &self.metric
    }

    /// Returns a reference to the set of samples of this time series.
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }
}

/// A single data point.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Sample {
    pub(crate) timestamp: f64,
    #[serde(deserialize_with = "de::deserialize_f64")]
    pub(crate) value: f64,
}

impl Sample {
    /// Returns the timestamp contained in this sample.
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    /// Returns the value contained in this sample.
    pub fn value(&self) -> f64 {
        self.value
    }
}

/// Collection of active and dropped targets as returned by the API.
#[derive(Debug, Deserialize)]
pub struct Targets {
    #[serde(alias = "activeTargets")]
    pub(crate) active: Vec<ActiveTarget>,
    #[serde(alias = "droppedTargets")]
    pub(crate) dropped: Vec<DroppedTarget>,
}

impl Targets {
    /// Get a list of currently active targets.
    pub fn active(&self) -> &[ActiveTarget] {
        &self.active
    }

    /// Get a list of dropped targets.
    pub fn dropped(&self) -> &[DroppedTarget] {
        &self.dropped
    }
}

/// A single active target.
#[derive(Debug, Deserialize)]
pub struct ActiveTarget {
    #[serde(alias = "discoveredLabels")]
    pub(crate) discovered_labels: HashMap<String, String>,
    pub(crate) labels: HashMap<String, String>,
    #[serde(alias = "scrapePool")]
    pub(crate) scrape_pool: String,
    #[serde(alias = "scrapeUrl")]
    #[serde(deserialize_with = "de::deserialize_url")]
    pub(crate) scrape_url: Url,
    #[serde(alias = "globalUrl")]
    #[serde(deserialize_with = "de::deserialize_url")]
    pub(crate) global_url: Url,
    #[serde(alias = "lastError")]
    pub(crate) last_error: String,
    #[serde(alias = "lastScrape")]
    #[serde(deserialize_with = "de::deserialize_rfc3339")]
    pub(crate) last_scrape: OffsetDateTime,
    #[serde(alias = "lastScrapeDuration")]
    pub(crate) last_scrape_duration: f64,
    pub(crate) health: TargetHealth,
    #[serde(alias = "scrapeInterval")]
    pub(crate) scrape_interval: String,
    #[serde(alias = "scrapeTimeout")]
    pub(crate) scrape_timeout: String,
}

impl ActiveTarget {
    /// Get a set of unmodified labels as before relabelling occurred.
    pub fn discovered_labels(&self) -> &HashMap<String, String> {
        &self.discovered_labels
    }

    /// Get a set of labels after relabelling.
    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }

    /// Get the scrape pool of this target.
    pub fn scrape_pool(&self) -> &str {
        &self.scrape_pool
    }

    /// Get the scrape URL of this target.
    pub fn scrape_url(&self) -> &Url {
        &self.scrape_url
    }

    /// Get the global URL of this target.
    pub fn global_url(&self) -> &Url {
        &self.global_url
    }

    /// Get the last error reported for this target.
    pub fn last_error(&self) -> &str {
        &self.last_error
    }

    /// Get the time when the last scrape occurred.
    pub fn last_scrape(&self) -> &OffsetDateTime {
        &self.last_scrape
    }

    /// Get the duration that the last scrape ran for in seconds.
    pub fn last_scrape_duration(&self) -> f64 {
        self.last_scrape_duration
    }

    /// Get the health status of this target.
    pub fn health(&self) -> TargetHealth {
        self.health
    }

    /// Get the scrape interval of this target.
    pub fn scrape_interval(&self) -> &str {
        &self.scrape_interval
    }

    /// Get the scrape timeout of this target.
    pub fn scrape_timeout(&self) -> &str {
        &self.scrape_timeout
    }
}

/// A single dropped target.
#[derive(Debug, Deserialize)]
pub struct DroppedTarget {
    #[serde(alias = "discoveredLabels")]
    pub(crate) discovered_labels: HashMap<String, String>,
}

impl DroppedTarget {
    /// Get a set of unmodified labels as before relabelling occurred.
    pub fn discovered_labels(&self) -> &HashMap<String, String> {
        &self.discovered_labels
    }
}

/// A group of rules.
#[derive(Debug, Deserialize)]
pub struct RuleGroup {
    pub(crate) rules: Vec<Rule>,
    pub(crate) file: String,
    pub(crate) interval: f64,
    pub(crate) name: String,
}

impl RuleGroup {
    /// Get a reference to all rules associated with this group.
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    /// Get the path to the file where this group is defined in.
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Get the interval that defines how often rules are evaluated.
    pub fn interval(&self) -> f64 {
        self.interval
    }

    /// Get the name of this rule group.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A wrapper for different types of rules that the HTTP API may return.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Rule {
    #[serde(alias = "recording")]
    Recording(RecordingRule),
    #[serde(alias = "alerting")]
    Alerting(AlertingRule),
}

/// An alerting rule.
#[derive(Debug, Deserialize)]
pub struct AlertingRule {
    pub(crate) alerts: Vec<Alert>,
    pub(crate) annotations: HashMap<String, String>,
    pub(crate) duration: f64,
    pub(crate) health: RuleHealth,
    pub(crate) labels: HashMap<String, String>,
    pub(crate) name: String,
    pub(crate) query: String,
}

impl AlertingRule {
    /// Get a list of active alerts fired due to this alerting rule.
    pub fn alerts(&self) -> &[Alert] {
        &self.alerts
    }

    /// Get a set of annotations set for this rule.
    pub fn annotations(&self) -> &HashMap<String, String> {
        &self.annotations
    }

    /// Get the duration that Prometheus waits for before firing for this rule.
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Get the health state of this rule.
    pub fn health(&self) -> RuleHealth {
        self.health
    }

    /// Get a set of labels defined for this rule.
    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }

    /// Get the name of this rule.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the PromQL expression that is evaluated as part of this rule.
    pub fn query(&self) -> &str {
        &self.query
    }
}

/// A recording rule.
#[derive(Debug, Deserialize)]
pub struct RecordingRule {
    pub(crate) health: RuleHealth,
    pub(crate) name: String,
    pub(crate) query: String,
    pub(crate) labels: Option<HashMap<String, String>>,
}

impl RecordingRule {
    /// Get the health state of this rule.
    pub fn health(&self) -> RuleHealth {
        self.health
    }

    /// Get the name of this rule.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the PromQL expression that is evaluated as part of this rule.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Get a set of labels defined for this rule.
    pub fn labels(&self) -> &Option<HashMap<String, String>> {
        &self.labels
    }
}

/// A single alert.
#[derive(Debug, Deserialize)]
pub struct Alert {
    #[serde(alias = "activeAt")]
    #[serde(deserialize_with = "de::deserialize_rfc3339")]
    pub(crate) active_at: OffsetDateTime,
    pub(crate) annotations: HashMap<String, String>,
    pub(crate) labels: HashMap<String, String>,
    pub(crate) state: AlertState,
    pub(crate) value: String,
}

impl Alert {
    /// Get the time when this alert started firing.
    pub fn active_at(&self) -> &OffsetDateTime {
        &self.active_at
    }

    /// Get a set of annotations associated with this alert.
    pub fn annotations(&self) -> &HashMap<String, String> {
        &self.annotations
    }

    /// Get a set of labels associated with this alert.
    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }

    /// Get the state of this alert.
    pub fn state(&self) -> AlertState {
        self.state
    }

    /// Get the value as evaluated by the PromQL expression that caused the alert to fire.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Collection of active and dropped alertmanagers as returned by the API.
#[derive(Debug)]
pub struct Alertmanagers {
    pub(crate) active: Vec<Url>,
    pub(crate) dropped: Vec<Url>,
}

impl Alertmanagers {
    /// Get a list of currently active alertmanagers.
    pub fn active(&self) -> &[Url] {
        &self.active
    }

    /// Get a list of dropped alertmanagers.
    pub fn dropped(&self) -> &[Url] {
        &self.dropped
    }
}

/// Possible metric types that the HTTP API may return.
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum MetricType {
    #[serde(alias = "counter")]
    Counter,
    #[serde(alias = "gauge")]
    Gauge,
    #[serde(alias = "histogram")]
    Histogram,
    #[serde(alias = "gaugehistogram")]
    GaugeHistogram,
    #[serde(alias = "summary")]
    Summary,
    #[serde(alias = "info")]
    Info,
    #[serde(alias = "stateset")]
    Stateset,
    #[serde(alias = "unknown")]
    Unknown,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
            MetricType::Histogram => write!(f, "histogram"),
            MetricType::GaugeHistogram => write!(f, "gaugehistogram"),
            MetricType::Summary => write!(f, "summary"),
            MetricType::Info => write!(f, "info"),
            MetricType::Stateset => write!(f, "stateset"),
            MetricType::Unknown => write!(f, "unknown"),
        }
    }
}

/// A target metadata object.
#[derive(Debug, Deserialize)]
pub struct TargetMetadata {
    pub(crate) target: HashMap<String, String>,
    #[serde(alias = "type")]
    pub(crate) metric_type: MetricType,
    pub(crate) metric: Option<String>,
    pub(crate) help: String,
    pub(crate) unit: String,
}

impl TargetMetadata {
    /// Get target labels.
    pub fn target(&self) -> &HashMap<String, String> {
        &self.target
    }

    /// Get the metric type.
    pub fn metric_type(&self) -> MetricType {
        self.metric_type
    }

    /// Get the metric name.
    pub fn metric(&self) -> Option<&str> {
        self.metric.as_deref()
    }

    /// Get the metric help.
    pub fn help(&self) -> &str {
        &self.help
    }

    /// Get the metric unit.
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

/// A metric metadata object
#[derive(Debug, Deserialize)]
pub struct MetricMetadata {
    #[serde(alias = "type")]
    pub(crate) metric_type: MetricType,
    pub(crate) help: String,
    pub(crate) unit: String,
}

impl MetricMetadata {
    /// Get the metric type.
    pub fn metric_type(&self) -> MetricType {
        self.metric_type
    }

    /// Get the metric help.
    pub fn help(&self) -> &str {
        &self.help
    }

    /// Get the metric unit.
    pub fn unit(&self) -> &str {
        &self.unit
    }
}
