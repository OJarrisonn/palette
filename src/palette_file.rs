use std::{error::Error, fmt::Debug};

use yaml_rust2::{Yaml, YamlLoader};

use crate::palette::Palette;

/// Create a `PaletteFile` from a file path
/// 
/// The file type is determined by the file extension, currently supporting `.toml` and `.json`
/// 
/// Unsupported extensions will return an `UnsupportedFile`, which will return an error when parsed
pub fn from_path(path: String) -> Box<dyn PaletteFile> {
    if path.ends_with(".toml") {
        Box::new(TomlFile(path.to_string())) as Box<dyn PaletteFile>
    } else if path.ends_with(".json") {
        Box::new(JsonFile(path.to_string())) as Box<dyn PaletteFile>
    } else if path.ends_with(".yaml") || path.ends_with(".yml") {
        Box::new(YamlFile(path.to_string())) as Box<dyn PaletteFile>
    } else {
        Box::new(UnsupportedFile(path.to_string())) as Box<dyn PaletteFile>
    }
}

pub trait PaletteFile: Debug {
    fn parse(&self) -> Result<Palette, Box<dyn Error>>;
}

/// A TOML file containing a color palette
///
/// Stores the path to the file
#[derive(Debug)]
pub struct TomlFile(String);

impl PaletteFile for TomlFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>> {
        let contents = std::fs::read_to_string(&self.0)?;
        let table: std::collections::HashMap<String, String> = toml::from_str(&contents)?;
        Palette::try_from(table).map_err(Into::into)
    }
}

/// A JSON file containing a color palette
/// 
/// Stores the path to the file
#[derive(Debug)]
pub struct JsonFile(String);

impl PaletteFile for JsonFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>> {
        let contents = std::fs::read_to_string(&self.0)?;
        let table: std::collections::HashMap<String, String> = serde_json::from_str(&contents)?;
        Palette::try_from(table).map_err(Into::into)
    }
}

/// A YAML file containing a color palette
///
/// Stores the path to the file
#[derive(Debug)]
pub struct YamlFile(String);

impl PaletteFile for YamlFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>> {
        let contents = std::fs::read_to_string(&self.0)?;
        let docs = YamlLoader::load_from_str(&contents)?;
        let doc = &docs[0];
        let mut table = std::collections::HashMap::new();
        if let Yaml::Hash(hash) = doc {
            for (key, value) in hash {
                if let (Yaml::String(k), Yaml::String(v)) = (key, value) {
                    table.insert(k.clone(), v.clone());
                }
            }
        }
        Palette::try_from(table).map_err(Into::into)
    }
}

/// An unsupported file type
///
/// Stores the path to the file
/// 
/// Will return an error when parsed
#[derive(Debug)]
pub struct UnsupportedFile(String);

impl PaletteFile for UnsupportedFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>> {
        Err(format!("Unsupported file type: {}", self.0).into())
    }
}
