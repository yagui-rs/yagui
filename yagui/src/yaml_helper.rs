use yaml_rust::{yaml, Yaml, YamlLoader};

use crate::error::{ensure, Result, YaguiError};

pub struct YamlHelper {
    yaml: Yaml,
}

impl YamlHelper {
    pub fn new(yaml: &str) -> Result<Self> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        ensure!(!docs.is_empty(), YaguiError::InvalidYaml);
        let yaml = docs.pop().unwrap();
        Ok(YamlHelper { yaml })
    }

    pub fn required_f64(&self, key: &str) -> Result<f64> {
        self.value_f64(key)
            .ok_or_else(|| YaguiError::MissingYamlValue(key.to_string(), "f64").into())
    }

    pub fn required_value(&self, key: &str) -> Result<&Yaml> {
        self.value(key)
            .ok_or_else(|| YaguiError::MissingYamlValue(key.to_string(), "object").into())
    }

    pub fn value(&self, key: &str) -> Option<&Yaml> {
        let keys: Vec<&str> = key.split('.').collect();
        let mut value = &self.yaml[keys[0]];
        for key in &keys[1..] {
            value = &value[*key];
        }
        if value.is_badvalue() {
            None
        } else {
            Some(value)
        }
    }

    pub fn value_f64(&self, key: &str) -> Option<f64> {
        let value = self.value(key);
        let f = value.and_then(|v| v.as_f64());
        let i = value.and_then(|v| v.as_i64());
        f.or(i.map(|v| v as f64))
    }

    pub fn value_str(&self, key: &str) -> Option<&str> {
        self.value(key).and_then(|v| v.as_str())
    }
}