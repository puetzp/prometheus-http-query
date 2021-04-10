pub trait Query {
    fn get_base_path(&self) -> &'static str;
    fn get_query_params(&self) -> Vec<(&str, &str)>;
}

pub struct InstantQuery<'a> {
    pub query: &'a str,
    pub time: Option<&'a str>,
    pub timeout: Option<&'a str>,
}

impl<'a> Query for InstantQuery<'a> {
    fn get_base_path(&self) -> &'static str {
        "/query"
    }

    fn get_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("query", self.query)];

        if let Some(t) = &self.time {
            params.push(("time", t));
        }

        if let Some(t) = &self.timeout {
            params.push(("timeout", t));
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
    fn get_base_path(&self) -> &'static str {
        "/query_range"
    }

    fn get_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("query", self.query)];

        if let Some(t) = &self.start {
            params.push(("start", t));
        }

        if let Some(t) = &self.end {
            params.push(("end", t));
        }
        if let Some(t) = &self.step {
            params.push(("step", t));
        }

        if let Some(t) = &self.timeout {
            params.push(("timeout", t));
        }

        params
    }
}
