use std::collections::HashMap;
use crate::http::query_string::Value::{Multiple, Single}; //because the enum is not exposed to the scope of other structs by default

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    // pub fn get(&self, key: &str) -> Option<&Value> {
    //     self.data.get(key)
    // }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {

    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(position) = key.find('=') {
                key = &sub_str[..position];
                val = &sub_str[(position + 1)..];


            }

            data.entry(key).and_modify(|v| {
                match v {
                    Single(prev) => {
                        let mut intro = Vec::new();
                        intro.push(val);
                        intro.push(prev);
                        *v = Multiple(intro);
                    },
                    Multiple(prev) => prev.push(val)
                }
            }).or_insert(Single(val));
        }

        QueryString { data }
    }
}