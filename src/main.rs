mod docgen;
mod footprints;
mod md;
mod symbols;

fn main() -> Result<(), std::io::Error> {
    //TODO: read file from arg
    let mut docs = symbols::build_docs(
        "/home/alexander/github.com/winterbloom_kicad_library/symbols/winterbloom.kicad_sym",
    )?;

    symbols::write_readme(
        "SYMBOLS.md",
        "Symbols",
        "symbol|reference|footprint",
        &mut docs,
    )?;

    let mut docs = footprints::build_docs(
        "/home/alexander/github.com/winterbloom_kicad_library/footprints/winterbloom.pretty",
    )?;
    footprints::write_readme("FOOTPRINTS.md", "Footprints", "footprint|step", &mut docs)?;

    Ok(())
}
