pub trait Query {
    fn get_base_path(&self) -> &'static str;
    fn get_query_params(&self) -> Vec<(&str, &str)>;
}

pub struct InstantQuery<'a> {
    pub query: &'a str,
    pub time: Option<&'a str>,
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

        params
    }
}
