use lexpr::parse::Error;
use std::fs::{self, File};

use crate::docgen::{self, DocItem};
use crate::md;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolDoc {
    pub symbol: String,
    pub reference: String,
    pub value: String,
    pub footprint: String,
    pub datasheet: String,
}

impl DocItem for SymbolDoc {
    fn elem(&self, el: &String) -> String {
        match el.as_str() {
            "symbol" => md::lexpr_str_to_md(self.symbol.clone()),
            "reference" => md::lexpr_str_to_md(self.reference.clone()),
            "value" => md::lexpr_str_to_md(self.value.clone()),
            "footprint" => md::lexpr_str_to_md(self.footprint.clone()),
            "datasheet" => format!(
                "[{}]({})",
                md::lexpr_str_to_md(self.symbol.clone()),
                md::lexpr_str_to_md(self.datasheet.clone())
            ),
            _ => String::new(),
        }
    }
}

impl SymbolDoc {
    pub fn sort_by_key(docs: &mut [SymbolDoc], format: &Vec<String>) {
        let first = format
            .first()
            .expect("Must hae at least one column")
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

#[cfg(test)]
mod tests {

    use std::fs;

    use lexpr::parse::Error;

    use crate::docgen::DocItem;
    use crate::symbols::SymbolDoc;

    #[test]
    fn elem() {
        let doc = SymbolDoc {
            symbol: "symbol".to_string(),
            reference: "reference".to_string(),
            value: "value".to_string(),
            footprint: "footprint".to_string(),
            datasheet: "datasheet".to_string(),
        };
        let elem = doc.elem(&"symbol".to_string());
        assert_eq!(elem, "symbol");
    }

    #[test]
    fn sort_by_key() {
        let mut docs = vec![
            SymbolDoc {
                symbol: "symbol".to_string(),
                reference: "reference".to_string(),
                value: "value".to_string(),
                footprint: "aootprint".to_string(),
                datasheet: "datasheet".to_string(),
            },
            SymbolDoc {
                symbol: "aymbol".to_string(),
                reference: "reference".to_string(),
                value: "value".to_string(),
                footprint: "footprint".to_string(),
                datasheet: "datasheet".to_string(),
            },
        ];
        SymbolDoc::sort_by_key(&mut docs, &vec!["symbol".to_string()]);
        assert_eq!(docs.first().expect("").symbol, "aymbol");
        SymbolDoc::sort_by_key(&mut docs, &vec!["footprint".to_string()]);
        assert_eq!(docs.first().expect("").footprint, "aootprint");
    }

    #[test]
    fn build_docs() -> Result<(), Error> {
        let docs = crate::symbols::build_docs("resources/test/lib.kicad_sym")?;
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].symbol, "\"74HC2G34\"");
        assert_eq!(docs[1].symbol, "\"XGZP6857D\"");
        assert_eq!(docs[0].reference, "\"U\"");
        assert_eq!(docs[1].reference, "\"U?\"");
        assert_eq!(docs[0].value, "\"74HC2G34\"");
        assert_eq!(docs[1].value, "\"XGZP6857D\"");
        assert_eq!(docs[0].footprint, "\"Package_TO_SOT_SMD:SOT-363_SC-70-6\"");
        assert_eq!(docs[1].footprint, "\"winterbloom:XGZP6857D\"");
        assert_eq!(
            docs[0].datasheet,
            "\"https://assets.nexperia.com/documents/data-sheet/74HC_HCT2G34.pdf\""
        );
        assert_eq!(docs[1].datasheet, "\"https://www.cfsensor.com/static/upload/file/20220412/XGZP6857D%20Pressure%20Sensor%20Module%20V2.4.pdf\"");
        Ok(())
    }

    #[test]
    fn write_readme() {
        let mut docs = crate::symbols::build_docs("resources/test/lib.kicad_sym")
            .expect("Test file lib.kicad_sym must exist");
        crate::symbols::write_readme(
            "Test Doc",
            "test.md",
            &Some(vec![
                "symbol".to_string(),
                "datasheet".to_string(),
                "footprint".to_string(),
            ]),
            &None,
            &mut docs,
        )
        .expect("Write should succeed");
        let file = fs::read_to_string("test.md").expect("Must be able to read test.md");
        const TEST_MD: &'static str = r#"# Test Doc

Symbol | Datasheet | Footprint
---|---|---
74HC2G34 | [74HC2G34](https://assets.nexperia.com/documents/data-sheet/74HC_HCT2G34.pdf) | Package_TO_SOT_SMD:SOT-363_SC-70-6
XGZP6857D | [XGZP6857D](https://www.cfsensor.com/static/upload/file/20220412/XGZP6857D%20Pressure%20Sensor%20Module%20V2.4.pdf) | winterbloom:XGZP6857D
"#;
        assert_eq!(file, TEST_MD);
        fs::remove_file("test.md").expect("Must be able to delete test.md");
    }
}
