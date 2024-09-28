use std::{collections::HashMap, fmt::Display};

use colored::Colorize;

use crate::{color::Color, palette};

#[derive(Debug, Default)]
pub struct Collection {
    names: Vec<Option<String>>,
    colors: HashMap<String, Vec<Option<Color>>>,
}

impl Collection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, pallete: palette::Palette) {
        self.names.push(pallete.name().map(String::from));

        for (key, value) in pallete.colors() {
            self.colors
                .entry(key.to_string())
                .or_default()
                .push(Some(*value));
        }
    }

    pub fn with_added(mut self, pallete: palette::Palette) -> Self {
        self.add(pallete);
        self
    }
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col_width = self
            .names
            .iter()
            .filter_map(|name| name.as_ref().map(|s| s.len()))
            .max()
            .map_or(7, |max_len| max_len.max(7));

        let head_width = self
            .colors
            .keys()
            .map(|s| s.len())
            .max()
            .map_or(5, |max_len| max_len.max(5));

        write!(f, "{:<head_width$} ", "name", head_width = head_width)?;

        for name in &self.names {
            write!(
                f,
                "| {:^col_width$} ",
                name.as_deref().unwrap_or(""),
                col_width = col_width
            )?;
        }

        writeln!(f)?;

        for (key, colors) in &self.colors {
            write!(f, "{:<head_width$} ", key, head_width = head_width)?;

            for color in colors {
                let elem = format!(
                    " {:^col_width$} ",
                    color.as_ref().map_or("".to_owned(), |c| c.to_string()),
                    col_width = col_width
                );

                match color {
                    None => write!(f, "|{}", elem)?,
                    Some(color@Color(r, g, b)) => write!(f, "|{}", 
                        if color.magnitude() < Color::THRESHOLD {
                            elem.white()
                        } else {
                            elem.black()
                        }.on_truecolor(*r, *g, *b)
                    )?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
