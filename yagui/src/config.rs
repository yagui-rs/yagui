use yaml_rust::{Yaml, YamlLoader};

use crate::error::{ensure, Result, YaguiError};

pub struct Config {
    yaml: Yaml,
}

impl Config {
    pub fn new(yaml: &Yaml) -> Self {
        Config { yaml: yaml.clone() }
    }

    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        ensure!(!docs.is_empty(), YaguiError::InvalidYaml);
        let yaml = docs.pop().unwrap();
        Ok(Config { yaml })
    }

    pub fn keys(&self) -> Vec<&str> {
        let mut result = Vec::new();
        if let Yaml::Hash(ref h) = self.yaml {
            for (k, _) in h {
                if let Yaml::String(s) = k {
                    result.push(s.as_str())
                }
            }
        }
        result
    }

    pub fn required_f64(&self, key: &str) -> Result<f64> {
        self.value_f64(key)
            .ok_or_else(|| YaguiError::MissingYamlValue(key.to_string(), "f64").into())
    }

    pub fn required_value(&self, key: &str) -> Result<&Yaml> {
        self.value(key)
            .ok_or_else(|| YaguiError::MissingYamlValue(key.to_string(), "object").into())
    }

    pub fn sub(&self, key: &str) -> Option<Config> {
        self.value(key).map(Self::new)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_f64() {
        let helper = Config::from_yaml(
            r"
            a: 1.0
            b: 5
            c:
              d: 4
              e: -50.1
            ",
        )
        .unwrap();

        assert_eq!(helper.value_f64("a"), Some(1.0));
        assert_eq!(helper.value_f64("b"), Some(5.0));
        assert_eq!(helper.value_f64("c.d"), Some(4.0));
        assert_eq!(helper.value_f64("c.e"), Some(-50.1));
        assert_eq!(helper.value_f64("c.f"), None);
        assert_eq!(helper.value_f64("g"), None);
    }
}
