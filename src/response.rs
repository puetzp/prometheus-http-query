use std::collections::HashMap;

pub enum Response {
    Vector(Vec<VectorSample>),
    Matrix(Vec<MatrixSample>),
}

#[derive(Debug, PartialEq)]
pub struct VectorSample {
    pub labels: HashMap<String, String>,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct MatrixSample {
    pub labels: HashMap<String, String>,
    pub values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub timestamp: f64,
    pub value: String,
}
