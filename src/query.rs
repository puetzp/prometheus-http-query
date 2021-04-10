pub trait Query {
    fn get_query_params(&self) -> Vec<(String, String)>;
}

pub struct InstantQuery<'a> {
    pub query: &'a str,
    pub time: Option<&'a str>,
    pub timeout: Option<&'a str>,
}

impl<'a> Query for InstantQuery<'a> {
    fn get_query_params(&self) -> Vec<(String, String)> {
        let mut params = vec![("query".to_string(), self.query.to_string())];

        if let Some(t) = &self.time {
            params.push(("time".to_string(), t.to_string()));
        }

        if let Some(t) = &self.timeout {
            params.push(("timeout".to_string(), t.to_string()));
        }

        params
    }
}

pub struct RangeQuery<'a> {
    pub query: &'a str,
    pub start: Option<&'a str>,
    pub end: Option<&'a str>,
    pub step: Option<&'a str>,
    pub timeout: Option<&'a str>,
}

impl<'a> Query for RangeQuery<'a> {
    fn get_query_params(&self) -> Vec<(String, String)> {
        let mut params = vec![("query".to_string(), self.query.to_string())];

        if let Some(t) = &self.start {
            params.push(("start".to_string(), t.to_string()));
        }

        if let Some(t) = &self.end {
            params.push(("end".to_string(), t.to_string()));
        }
        if let Some(t) = &self.step {
            params.push(("step".to_string(), t.to_string()));
        }

        if let Some(t) = &self.timeout {
            params.push(("timeout".to_string(), t.to_string()));
        }

        params
    }
}
