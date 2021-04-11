pub trait Query {
    fn get_query_params(&self) -> Vec<(&str, &str)>;
}

pub struct InstantQuery<'a> {
    pub query: &'a str,
    pub time: Option<&'a str>,
    pub timeout: Option<&'a str>,
}

impl<'a> Query for InstantQuery<'a> {
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
    pub start: &'a str,
    pub end: &'a str,
    pub step: &'a str,
    pub timeout: Option<&'a str>,
}

impl<'a> Query for RangeQuery<'a> {
    fn get_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![
            ("query", self.query),
            ("start", self.start),
            ("end", self.end),
            ("step", self.step),
        ];

        if let Some(t) = &self.timeout {
            params.push(("timeout", t));
        }

        params
    }
}
