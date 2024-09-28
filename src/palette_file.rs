use std::error::Error;

use crate::palette::Palette;

pub fn from_path(path: String) -> Box<dyn PaletteFile> {
    if path.ends_with(".toml") {
        Box::new(TomlFile(path.to_string())) as Box<dyn PaletteFile>
    } else {
        Box::new(UnsupportedFile(path.to_string())) as Box<dyn PaletteFile>
    }
}

pub trait PaletteFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>>;
}

/// A TOML file containing a color palette
///
/// Stores the path to the file
pub struct TomlFile(String);

impl PaletteFile for TomlFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>> {
        let contents = std::fs::read_to_string(&self.0)?;
        let table: std::collections::HashMap<String, String> = toml::from_str(&contents)?;
        Palette::try_from(table).map_err(Into::into)
    }
}

/// An unsupported file type
///
/// Stores the path to the file
pub struct UnsupportedFile(String);

impl PaletteFile for UnsupportedFile {
    fn parse(&self) -> Result<Palette, Box<dyn Error>> {
        Err(format!("Unsupported file type: {}", self.0).into())
    }
}
