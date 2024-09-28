use std::error::Error;

use clap::Parser;
use collection::Collection;
use palette::Palette;

mod collection;
mod color;
mod palette;
mod palette_file;

#[derive(Parser, Debug)]
#[clap(author, version)]
/// palette is a CLI tool that helps you to visualize your color palettes on the terminal.
/// Truecolor support is required.
///
/// The tool reads .toml, .yaml/yml, .json and .nuon files containing color palettes and displays them in a table for easy comparison.
struct Cli {
    /// The files containing the palettes to be shown
    #[clap(required = true)]
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let files = args
        .files
        .into_iter()
        .map(palette_file::from_path)
        .collect::<Vec<_>>();

    let palettes = files
        .into_iter()
        .map(|file| file.parse())
        .collect::<Result<Vec<Palette>, _>>()?;

    let collection = palettes
        .into_iter()
        .fold(Collection::new(), |c, p| c.with_added(p));

    println!("{}", collection);

    Ok(())
}
