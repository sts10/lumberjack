use std::mem;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Features {
    vec: Vec<(String, Option<String>)>,
}

impl<S> From<S> for Features
    where
        S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let mut vec = Vec::new();
        for f in s.as_ref().split('|') {
            if let Some(idx) = f.find(':') {
                let (k, v) = f.split_at(idx);
                vec.push((k.into(), Some(v.into())))
            } else {
                vec.push((f.into(), None))
            }
        }
        Features { vec }
    }
}

impl Features {
    pub fn from_vec(vec: Vec<(String, Option<String>)>) -> Self {
        Features { vec }
    }

    pub fn inner(&self) -> &[(String, Option<String>)] {
        &self.vec
    }

    pub fn inner_mut(&mut self) -> &mut Vec<(String, Option<String>)> {
        &mut self.vec
    }

    pub fn insert(
        &mut self,
        key: impl AsRef<str>,
        val: Option<impl Into<String>>,
    ) -> Option<String> {
        let key = key.as_ref();
        let val = val.map(|s| s.into());
        for i in 0..self.vec.len() {
            if self.vec[i].0 == key {
                return mem::replace(&mut self.vec[i].1, val)
            }
        }
        self.vec.push((key.into(), val));
        None
    }

    pub fn get_val(&self, key: &str) -> Option<&str> {
        self.vec.iter().find_map(|(k, v)| {
            if key == k.as_str() {
                v.as_ref().map(|s| s.as_str())
            } else {
                None
            }
        })
    }
}

impl ToString for Features {
    fn to_string(&self) -> String {
        self.vec
            .iter()
            .map(|(k, v)| {
                if let Some(v) = v {
                    format!("{}:{}", k, v)
                } else {
                    k.to_owned()
                }
            })
            .join("|")
    }
}