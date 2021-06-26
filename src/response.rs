//! Collection of response types, most importantly the [Response] enum
use crate::util::{AlertState, RuleHealth, TargetHealth};
use serde::Deserialize;
use std::collections::HashMap;

/// A wrapper for all kinds of responses the API returns.
#[derive(Debug)]
pub enum Response {
    Instant(Vec<Vector>),
    Range(Vec<Matrix>),
    Series(Vec<HashMap<String, String>>),
    LabelName(Vec<String>),
    LabelValue(Vec<String>),
    Target(Targets),
    Rule(Vec<RuleGroup>),
    Alert(Vec<Alert>),
    Flags(HashMap<String, String>),
}

impl Response {
    /// If the `Response`'s contains instant vectors, returns an array of [Vector]s. Returns `None` otherwise.
    pub fn as_instant(&self) -> Option<&[Vector]> {
        match self {
            Response::Instant(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response`'s contains range vectors, returns an array of [Matrix]s. Returns `None` otherwise.
    pub fn as_range(&self) -> Option<&[Matrix]> {
        match self {
            Response::Range(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response` contains time series, returns an array of time series. Returns `None` otherwise.
    pub fn as_series(&self) -> Option<&[HashMap<String, String>]> {
        match self {
            Response::Series(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response` contains a collection of label names, returns an array of strings. Returns `None` otherwise.
    pub fn as_label_name(&self) -> Option<&[String]> {
        match self {
            Response::LabelName(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response` contains a collection of label values, returns an array of strings. Returns `None` otherwise.
    pub fn as_label_value(&self) -> Option<&[String]> {
        match self {
            Response::LabelValue(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response` contains a set of active or dropped targets, returns [Targets]. Returns `None` otherwise.
    pub fn as_target(&self) -> Option<&Targets> {
        match self {
            Response::Target(t) => Some(&t),
            _ => None,
        }
    }

    /// If the `Response` contains a set of rule groups, returns an array of [Group]s. Returns `None` otherwise.
    pub fn as_rule(&self) -> Option<&[RuleGroup]> {
        match self {
            Response::Rule(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response` contains a set of alerts, returns an array of [Alert]s. Returns `None` otherwise.
    pub fn as_alert(&self) -> Option<&[Alert]> {
        match self {
            Response::Alert(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    /// If the `Response` contains flags, returns a map with flag names as keys. Returns `None` otherwise.
    pub fn as_flags(&self) -> Option<&HashMap<String, String>> {
        match self {
            Response::Flags(f) => Some(&f),
            _ => None,
        }
    }
}

/// A single time series containing a single data point ([Sample]).
#[derive(Debug, PartialEq, Deserialize)]
pub struct Vector {
    pub(crate) metric: HashMap<String, String>,
    #[serde(alias = "value")]
    pub(crate) sample: Sample,
}

impl Vector {
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

/// A single time series containing a range of data points ([Sample]s).
#[derive(Debug, PartialEq, Deserialize)]
pub struct Matrix {
    pub(crate) metric: HashMap<String, String>,
    #[serde(alias = "values")]
    pub(crate) samples: Vec<Sample>,
}

impl Matrix {
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
    pub(crate) value: String,
}

impl Sample {
    /// Returns the timestamp contained in this sample.
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    /// Returns the value contained in this sample.
    pub fn value(&self) -> &str {
        &self.value
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

/// A single active target.v
#[derive(Debug, Deserialize)]
pub struct ActiveTarget {
    #[serde(alias = "discoveredLabels")]
    pub(crate) discovered_labels: HashMap<String, String>,
    pub(crate) labels: HashMap<String, String>,
    #[serde(alias = "scrapePool")]
    pub(crate) scrape_pool: String,
    #[serde(alias = "scrapeUrl")]
    pub(crate) scrape_url: String,
    #[serde(alias = "globalUrl")]
    pub(crate) global_url: String,
    #[serde(alias = "lastError")]
    pub(crate) last_error: String,
    #[serde(alias = "lastScrape")]
    pub(crate) last_scrape: time::OffsetDateTime,
    #[serde(alias = "lastScrapeDuration")]
    pub(crate) last_scrape_duration: f64,
    pub(crate) health: TargetHealth,
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
    pub fn scrape_url(&self) -> &str {
        &self.scrape_url
    }

    /// Get the global URL of this target.
    pub fn global_url(&self) -> &str {
        &self.global_url
    }

    /// Get the last error reported for this target.
    pub fn last_error(&self) -> &str {
        &self.last_error
    }

    /// Get the timestamp of the last scrape in RFC3339 format.
    pub fn last_scrape(&self) -> &time::OffsetDateTime {
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

/// A wrapper enum for different rule types.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Rule {
    #[serde(alias = "recording")]
    Recording(RecordingRule),
    #[serde(alias = "alerting")]
    Alerting(AlertingRule),
}

/// A recording rule.
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

/// An alerting rule.
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

#[derive(Debug, Deserialize)]
pub struct Alert {
    #[serde(alias = "activeAt")]
    pub(crate) active_at: time::OffsetDateTime,
    pub(crate) annotations: HashMap<String, String>,
    pub(crate) labels: HashMap<String, String>,
    pub(crate) state: AlertState,
    pub(crate) value: String,
}

impl Alert {
    /// Get the timestamp (RFC3339 formatted).
    pub fn active_at(&self) -> &time::OffsetDateTime {
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
