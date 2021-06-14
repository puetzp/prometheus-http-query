use std::collections::HashMap;

pub enum Response {
    Vector(Vec<Vector>),
    Matrix(Vec<Matrix>),
}

#[derive(Debug, PartialEq)]
pub struct Vector {
    pub(crate) labels: HashMap<String, String>,
    pub(crate) sample: Sample,
}

impl Vector {
    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub(crate) labels: HashMap<String, String>,
    pub(crate) samples: Vec<Sample>,
}

#[derive(Debug, PartialEq)]
pub struct Sample {
    pub(crate) timestamp: f64,
    pub(crate) value: String,
}
