use std::{collections::HashMap, str::FromStr};

use toml::{map::Map, Value};

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Palette {
    name: Option<String>,
    colors: HashMap<String, Color>,
}

impl Palette {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn colors(&self) -> &HashMap<String, Color> {
        &self.colors
    }
}

impl TryFrom<Map<String, Value>> for Palette {
    type Error = &'static str;

    fn try_from(mut table: Map<String, Value>) -> Result<Self, Self::Error> {
        let mut name = None;
        let mut colors = HashMap::new();

        // Convert the keys to lowercase
        table = table
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v))
            .collect();

        if let Some(Value::String(n)) = table.remove("name") {
            name = Some(n);
        }

        for (key, value) in table {
            if let Value::String(hex) = value {
                let rgb = Color::from_str(&hex).map_err(|_| "Invalid color")?;
                colors.insert(key.to_lowercase(), rgb);
            } else {
                return Err("The value of the key is not a string");
            }
        }

        Ok(Palette { name, colors })
    }
}
