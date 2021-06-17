use std::collections::HashMap;

/// A response wrapper for vector and matrix return types.
/// The [Vector] and [Matrix] types encapsulate a set of time series with a single sample
/// and a set of time series containing a range of sample data respectively.
#[derive(Debug)]
pub enum Response {
    Vector(Vec<Vector>),
    Matrix(Vec<Matrix>),
    Series(Vec<HashMap<String, String>>),
    Labels(Vec<String>),
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
