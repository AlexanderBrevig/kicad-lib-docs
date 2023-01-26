use clap::{Parser, Subcommand};
use std::error::Error;

mod docgen;
mod footprints;
mod md;
mod symbols;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create README for symbols
    Symbols {
        /// Title of README
        title: String,
        /// Path to input kicad_sym file
        in_file: String,
        /// Path to output README.md file
        out_file: String,
        /// Add multiple columns. Default is `-c symbol -c footprint -c datasheet`
        #[arg(short, long, verbatim_doc_comment, action = clap::ArgAction::Append,  value_parser = clap::builder::PossibleValuesParser::new(["symbol", "reference", "footprint", "datasheet", "value"]))]
        column: Option<Vec<String>>,
        /// ENV is key=value, use to replace paths for datasheets
        #[arg(short, long,value_parser = parse_key_val::<String, String>)]
        env: Option<Vec<(String, String)>>,
    },
    /// Create README for footprints
    Footprints {
        /// Title of README
        title: String,
        /// Path to input kicad_sym file
        in_file: String,
        /// Path to output README.md file
        out_file: String,
        /// Add multiple columns. Default is `-c footprint -c step`
        #[arg(short, long, verbatim_doc_comment, action = clap::ArgAction::Append,  value_parser = clap::builder::PossibleValuesParser::new(["footprint", "step"]))]
        column: Option<Vec<String>>,
        /// ENV is key=value, use to replace paths for datasheets
        #[arg(short, long,value_parser = parse_key_val::<String, String>)]
        env: Option<Vec<(String, String)>>,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Symbols {
            column,
            env,
            title,
            in_file,
            out_file,
        }) => {
            let mut docs = symbols::build_docs(in_file)?;
            symbols::write_readme(title, out_file, column, env, &mut docs)?;
        }

        Some(Commands::Footprints {
            column,
            env,
            title,
            in_file,
            out_file,
        }) => {
            let mut docs = footprints::build_docs(in_file)?;
            footprints::write_readme(title, out_file, column, env, &mut docs)?;
        }

        None => { /* Default action is to print the --help */ }
    }

    Ok(())
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
