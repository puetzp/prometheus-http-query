use std::collections::HashMap;

#[derive(Debug)]
pub enum Response {
    Vector(Vec<Vector>),
    Matrix(Vec<Matrix>),
}

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
