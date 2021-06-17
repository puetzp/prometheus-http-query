//! Collection of response types, most importantly the [Response] enum
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

/// A single active target.
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
    pub(crate) health: String,
}

/// A single dropped target.
#[derive(Debug)]
pub struct DroppedTarget(pub(crate) HashMap<String, String>);
