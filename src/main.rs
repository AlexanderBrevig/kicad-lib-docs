use lexpr::{parse::Error, Value};
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug)]
struct SymbolDoc {
    symbol: String,
    reference: String,
    value: String,
    footprint: String,
    datasheet: String,
}

impl SymbolDoc {
    fn elem(&self, el: &str) -> String {
        match el {
            "symbol" => self.symbol.clone(),
            "reference" => self.reference.clone(),
            "value" => self.value.clone(),
            "footprint" => self.footprint.clone(),
            "datasheet" => self.datasheet.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Debug)]
struct FootprintDoc {
    footprint: String,
    step: String,
}

impl FootprintDoc {
    fn elem(&self, el: &str) -> String {
        match el {
            "footprint" => self.footprint.clone(),
            "step" => self.step.clone(),
            _ => String::new(),
        }
    }
}

fn find(val: &Value, root: &str, qualifier: &str, offset: usize) -> String {
    let mut root_idx = 0;
    loop {
        if val[root_idx].is_nil() {
            break;
        }
        if val[root_idx][0].to_string() == root
            && val[root_idx][1].to_string() == format!("\"{}\"", qualifier)
        {
            //&& val[root_idx][1].to_string() == qualifier {
            return val[root_idx][offset].to_string().replace("\"", "");
        }
        root_idx += 1;
    }
    return "#nil".to_string();
}

fn build_symbol_docs(file: &str) -> Result<Vec<SymbolDoc>, Error> {
    let data = fs::read_to_string(file).expect("Unable to read file");

    let kicad_sym = lexpr::from_str(&data)?;

    let mut docs: Vec<SymbolDoc> = vec![];
    let mut sym_idx = 0;
    const SKIP_HEADER: usize = 3;
    loop {
        let doc = SymbolDoc {
            symbol: kicad_sym[SKIP_HEADER + sym_idx][1]
                .to_string()
                .replace("\"", ""),
            reference: find(
                &kicad_sym[SKIP_HEADER + sym_idx],
                "property",
                "Reference",
                2,
            ),
            value: find(&kicad_sym[SKIP_HEADER + sym_idx], "property", "Value", 2),
            footprint: find(
                &kicad_sym[SKIP_HEADER + sym_idx],
                "property",
                "Footprint",
                2,
            ),
            datasheet: find(
                &kicad_sym[SKIP_HEADER + sym_idx],
                "property",
                "Datasheet",
                2,
            ),
        };
        if doc.symbol == "#nil" {
            break;
        }
        docs.push(doc);
        sym_idx += 1;
    }

    Ok(docs)
}

fn write_symbols_readme(
    file: &str,
    title: &str,
    format: &str,
    docs: Vec<SymbolDoc>,
) -> Result<(), std::io::Error> {
    let mut w = File::create(file).unwrap();
    write!(&mut w, "# {}\n\n", title)?;
    let mut first = true;
    for elem in format.split("|") {
        if !first {
            write!(&mut w, " | ")?;
        }
        write!(
            &mut w,
            "{}{}",
            (&elem[..1].to_string()).to_uppercase(),
            &elem[1..]
        )?;
        first = false;
    }
    writeln!(&mut w, "")?;
    for _ in format.split("|") {
        if !first {
            write!(&mut w, "|")?;
        }
        write!(&mut w, "---")?;
        first = false;
    }
    writeln!(&mut w, "")?;
    for doc in docs {
        first = true;
        for elem in format.split("|") {
            if !first {
                write!(&mut w, " | ")?;
            }
            write!(&mut w, "{}", doc.elem(elem))?;
            first = false;
        }
        writeln!(&mut w, "")?;
    }
    Ok(())
}

fn build_footprint_docs(folder: &str) -> Result<Vec<FootprintDoc>, std::io::Error> {
    let mut docs: Vec<FootprintDoc> = vec![];
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        println!("{:?}", path);
        let data = fs::read_to_string(path).expect("Unable to read file");

        let kicad_sym = lexpr::from_str(&data)?;

        let doc = FootprintDoc {
            footprint: kicad_sym[1].to_string().replace("\"", ""),
            //TODO: replace kicadmod with path
            step: kicad_sym["model"][0].to_string(),
        };
        docs.push(doc);
    }

    Ok(docs)
}

fn write_footprints_readme(
    file: &str,
    title: &str,
    format: &str,
    docs: Vec<FootprintDoc>,
) -> Result<(), std::io::Error> {
    let mut w = File::create(file).unwrap();
    write!(&mut w, "# {}\n\n", title)?;
    let mut first = true;
    for elem in format.split("|") {
        if !first {
            write!(&mut w, " | ")?;
        }
        write!(
            &mut w,
            "{}{}",
            (&elem[..1].to_string()).to_uppercase(),
            &elem[1..]
        )?;
        first = false;
    }
    writeln!(&mut w, "")?;
    for _ in format.split("|") {
        if !first {
            write!(&mut w, "|")?;
        }
        write!(&mut w, "---")?;
        first = false;
    }
    writeln!(&mut w, "")?;
    for doc in docs {
        first = true;
        for elem in format.split("|") {
            if !first {
                write!(&mut w, " | ")?;
            }
            write!(&mut w, "{}", doc.elem(elem))?;
            first = false;
        }
        writeln!(&mut w, "")?;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    //TODO: read file from arg
    let docs = build_symbol_docs(
        "/home/alexander/github.com/winterbloom_kicad_library/symbols/winterbloom.kicad_sym",
    )?;

    write_symbols_readme("SYMBOLS.md", "Symbols", "reference|symbol", docs)?;

    let docs = build_footprint_docs(
        "/home/alexander/github.com/winterbloom_kicad_library/footprints/winterbloom.pretty",
    )?;
    write_footprints_readme("FOOTPRINTS.md", "Footprints", "footprint|step", docs)?;

    Ok(())
}
