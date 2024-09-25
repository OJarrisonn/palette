use std::{collections::HashMap, error::Error};

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file)?;

    let parsed_toml: toml::Value = toml::from_str(&content)?;

    let mut name = None;
    let mut colors = HashMap::new();
    let max_key_length;

    if let toml::Value::Table(mut table) = parsed_toml {
        max_key_length = table.keys().map(|k| k.len()).max().unwrap_or(0);

        if let Some(toml::Value::String(n)) = table.remove("name") {
            name = Some(n);
        }

        for (key, value) in table {
            if let toml::Value::String(hex) = value {
                let rgb = parse_hex(&hex)?;
                colors.insert(key.to_lowercase(), (rgb, hex));
            } else {
                panic!("The value of the key {} is not a string", key);
            }
        }
    } else {
        panic!("The root of the TOML file is not a table");
    }

    if let Some(name) = name {
        println!(" {} ", name);
    }

    for (key, value) in colors {
        println!(
            "{}",
            format!(" {:width$} | {} ", key, value.1, width = max_key_length)
                .on_truecolor(value.0 .0, value.0 .1, value.0 .2)
        );
    }

    Ok(())
}

/// Parse a hex String `#RRGGBB` into a tuple of `u8` RGB values `(r, g, b)`
/// 
/// # Errors
/// 
/// Returns an error if the hex string is not a valid hex color
/// 
/// # Panics 
/// 
/// Panics if the hex string is not 7 characters long (`"#RRGGBB"`)
fn parse_hex(hex: &str) -> Result<(u8, u8, u8), Box<dyn Error>> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}
