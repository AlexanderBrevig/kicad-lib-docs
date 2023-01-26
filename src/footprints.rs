use crate::docgen::DocItem;
use crate::md;
use regex::Regex;
use std::fs::{self, File};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FootprintDoc {
    pub footprint: String,
    pub step: String,
}

impl DocItem for FootprintDoc {
    fn elem(&self, el: &String) -> String {
        match el.as_str() {
            "footprint" => md::lexpr_str_to_md(self.footprint.clone()),
            "step" => md::lexpr_str_to_md(self.step.clone()),
            _ => String::new(),
        }
    }
}

impl FootprintDoc {
    fn sort_by_key(docs: &mut [FootprintDoc], format: &Vec<String>) {
        let first = format
            .first()
            .expect("Format must contain at least one key");
        match first.as_str() {
            "footprint" => docs.sort_by_key(|doc| doc.footprint.clone()),
            "step" => docs.sort_by_key(|doc| doc.step.clone()),
            _ => {}
        }
    }
}

pub fn build_docs(folder: &str) -> Result<Vec<FootprintDoc>, std::io::Error> {
    let mut docs: Vec<FootprintDoc> = vec![];
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let data = fs::read_to_string(path).expect("Unable to read file");

        let re = Regex::new(r"\((tstamp|tedit) [a-zA-Z0-9-]*\)").unwrap();
        let data = re.replace_all(&data, "");
        let kicad_sym = lexpr::from_str(&data)?;

        let doc = FootprintDoc {
            footprint: kicad_sym[1].to_string().to_string(),
            step: kicad_sym["model"][0].to_string(),
        };
        docs.push(doc);
    }

    Ok(docs)
}

pub fn write_readme(
    title: &str,
    file: &str,
    format: &Option<Vec<String>>,
    _env: &Option<Vec<(String, String)>>, //TODO: handle env
    footprint_docs: &mut Vec<FootprintDoc>,
) -> Result<(), std::io::Error> {
    let default = vec!["footprint".to_string(), "step".to_string()];
    let format = format.as_ref().unwrap_or(&default);
    FootprintDoc::sort_by_key(footprint_docs, &format);
    let mut writer = File::create(file).unwrap();
    md::title(&mut writer, title)?;
    md::table_header(&mut writer, &format)?;
    md::table_sep(&mut writer, &format)?;
    md::table_content(&mut writer, &format, footprint_docs)?;
    Ok(())
}
