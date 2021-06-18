//! Collection of response types, most importantly the [Response] enum
use crate::util::TargetHealth;
use std::collections::HashMap;

/// A wrapper for any kind of response the API returns.
#[derive(Debug)]
pub enum Response {
    Vector(Vec<Vector>),
    Matrix(Vec<Matrix>),
    Series(Vec<HashMap<String, String>>),
    LabelNames(Vec<String>),
    LabelValues(Vec<String>),
    Targets(Targets),
}

/// A single time series containing a single data point ([Sample]).
#[derive(Debug, PartialEq)]
pub struct Vector {
    pub(crate) metric: HashMap<String, String>,
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
#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub(crate) metric: HashMap<String, String>,
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
pub struct Targets {
    pub(crate) active: Vec<ActiveTarget>,
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
#[derive(Debug)]
pub struct ActiveTarget {
    pub(crate) discovered_labels: HashMap<String, String>,
    pub(crate) labels: HashMap<String, String>,
    pub(crate) scrape_pool: String,
    pub(crate) scrape_url: String,
    pub(crate) global_url: String,
    pub(crate) last_error: String,
    pub(crate) last_scrape: String,
    pub(crate) last_scrape_duration: f64,
    pub(crate) health: TargetHealth,
}

impl ActiveTarget {
    /// Get a list of unmodified labels as before relabelling occurred.
    pub fn discovered_labels(&self) -> &HashMap<String, String> {
        &self.discovered_labels
    }

    /// Get a list of labels after relabelling.
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
    pub fn last_scrape(&self) -> &str {
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
#[derive(Debug)]
pub struct DroppedTarget(pub(crate) HashMap<String, String>);

impl DroppedTarget {
    /// Get a list of unmodified labels as before relabelling occurred.
    pub fn discovered_labels(&self) -> &HashMap<String, String> {
        let DroppedTarget(discovered_labels) = self;
        discovered_labels
    }
}
