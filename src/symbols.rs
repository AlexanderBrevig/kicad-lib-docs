use lexpr::parse::Error;
use std::fs::{self, File};

use crate::docgen::{self, DocItem};
use crate::md;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolDoc {
    symbol: String,
    reference: String,
    value: String,
    footprint: String,
    datasheet: String,
}

impl DocItem for SymbolDoc {
    fn elem(&self, el: &String) -> String {
        match el.as_str() {
            "symbol" => md::lexpr_str_to_md(self.symbol.clone()),
            "reference" => md::lexpr_str_to_md(self.reference.clone()),
            "value" => md::lexpr_str_to_md(self.value.clone()),
            "footprint" => md::lexpr_str_to_md(self.footprint.clone()),
            "datasheet" => md::lexpr_str_to_md(self.datasheet.clone()),
            _ => String::new(),
        }
    }
}

impl SymbolDoc {
    fn sort_by_key(docs: &mut [SymbolDoc], format: &Vec<String>) {
        let first = format
            .first()
            .expect("Must have at least one column")
            .as_str();
        match first {
            "symbol" => docs.sort_by_key(|doc| doc.symbol.clone()),
            "reference" => docs.sort_by_key(|doc| doc.reference.clone()),
            "value" => docs.sort_by_key(|doc| doc.value.clone()),
            "footprint" => docs.sort_by_key(|doc| doc.footprint.clone()),
            "datasheet" => docs.sort_by_key(|doc| doc.datasheet.clone()),
            _ => {}
        }
    }
}
pub fn build_docs(file: &str) -> Result<Vec<SymbolDoc>, Error> {
    let data = fs::read_to_string(file).expect("Unable to read file");

    let kicad_sym = lexpr::from_str(&data)?;

    let mut docs: Vec<SymbolDoc> = vec![];
    let mut sym_idx = 0;
    const SKIP_HEADER: usize = 3;
    loop {
        let doc = SymbolDoc {
            symbol: kicad_sym[SKIP_HEADER + sym_idx][1].to_string(),
            reference: docgen::find(
                &kicad_sym[SKIP_HEADER + sym_idx],
                "property",
                "Reference",
                2,
            ),
            value: docgen::find(&kicad_sym[SKIP_HEADER + sym_idx], "property", "Value", 2),
            footprint: docgen::find(
                &kicad_sym[SKIP_HEADER + sym_idx],
                "property",
                "Footprint",
                2,
            ),
            datasheet: docgen::find(
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

pub fn write_readme(
    title: &str,
    file: &str,
    format: &Option<Vec<String>>,
    _env: &Option<Vec<(String, String)>>, //TODO: handle env
    symbol_docs: &mut Vec<SymbolDoc>,
) -> Result<(), std::io::Error> {
    let default = vec![
        "symbol".to_string(),
        "datasheet".to_string(),
        "footprint".to_string(),
    ];
    let format = format.as_ref().unwrap_or(&default);
    SymbolDoc::sort_by_key(symbol_docs, &format);
    let mut writer = File::create(file).unwrap();
    md::title(&mut writer, title)?;
    md::table_header(&mut writer, &format)?;
    md::table_sep(&mut writer, &format)?;
    md::table_content(&mut writer, &format, symbol_docs)?;
    Ok(())
}
