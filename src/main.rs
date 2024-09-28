use std::error::Error;

use clap::Parser;
use collection::Collection;
use toml::map::Map;

mod collection;
mod color;
mod palette;

#[derive(Parser, Debug)]
#[clap(author, version)]
/// palette is a CLI tool that helps you to visualize your color palettes on the terminal.
/// Truecolor support is required.
/// 
/// The tool reads .toml files containing color palettes and displays them in a table for easy comparison.
struct Cli {
    /// The .toml files containing the palettes to be shown
    #[clap(required = true)]
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let contents = args
        .files
        .into_iter()
        .map(std::fs::read_to_string)
        .collect::<Result<Vec<_>, _>>()?;

    let maps = contents
        .into_iter()
        .map(|c| toml::from_str(&c))
        .collect::<Result<Vec<Map<String, toml::Value>>, _>>()?;

    let palettes = maps
        .into_iter()
        .map(palette::Palette::try_from)
        .collect::<Result<Vec<palette::Palette>, _>>()?;

    let collection = palettes
        .into_iter()
        .fold(Collection::new(), |c, p| c.with_added(p));

    println!("{}", collection);

    Ok(())
}
