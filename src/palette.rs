use std::{collections::HashMap, str::FromStr};

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

impl TryFrom<HashMap<String, String>> for Palette {
    type Error = &'static str;

    fn try_from(mut table: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut name = None;
        let mut colors = HashMap::new();

        // Convert the keys to lowercase
        table = table
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v))
            .collect();

        if let Some(n) = table.remove("name") {
            name = Some(n);
        }

        for (key, hex) in table {
            let rgb = Color::from_str(&hex).map_err(|_| "Invalid color")?;
            colors.insert(key.to_lowercase(), rgb);
        }

        Ok(Palette { name, colors })
    }
}
