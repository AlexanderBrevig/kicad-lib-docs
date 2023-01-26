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

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::docgen::DocItem;
    use crate::footprints::FootprintDoc;

    #[test]
    fn elem() {
        let doc = FootprintDoc {
            footprint: "footprint".to_string(),
            step: "step".to_string(),
        };
        let elem = doc.elem(&"footprint".to_string());
        assert_eq!(elem, "footprint");
    }

    #[test]
    fn sort_by_key() {
        let mut docs = vec![
            FootprintDoc {
                footprint: "footprint".to_string(),
                step: "atep".to_string(),
            },
            FootprintDoc {
                footprint: "aootprint".to_string(),
                step: "step".to_string(),
            },
        ];
        FootprintDoc::sort_by_key(&mut docs, &vec!["footprint".to_string()]);
        assert_eq!(docs.first().expect("").footprint, "aootprint");
        FootprintDoc::sort_by_key(&mut docs, &vec!["step".to_string()]);
        assert_eq!(docs.first().expect("").step, "atep");
    }

    #[test]
    fn build_docs() -> Result<(), std::io::Error> {
        let docs = crate::footprints::build_docs("resources/test/lib.pretty")?;
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].footprint, "\"AudioJack_WQP518MA\"");
        assert_eq!(docs[1].footprint, "\"D_SMA\"");
        assert_eq!(docs[0].step, "\"${WINTERBLOOM3DMOD}/WQP-WQP518MA.step\"");
        assert_eq!(
            docs[1].step,
            "\"${KICAD6_3DMODEL_DIR}/Diode_SMD.3dshapes/D_SMA.wrl\""
        );
        Ok(())
    }

    #[test]
    fn write_readme() {
        let mut docs = crate::footprints::build_docs("resources/test/lib.pretty")
            .expect("Test folder lib.pretty must exist");
        crate::footprints::write_readme(
            "Test Doc",
            "test.md",
            &Some(vec!["footprint".to_string(), "step".to_string()]),
            &None,
            &mut docs,
        )
        .expect("Write should succeed");
        let file = fs::read_to_string("test.md").expect("Must be able to read test.md");
        const TEST_MD: &'static str = r#"# Test Doc

Footprint | Step
---|---
AudioJack_WQP518MA | ${WINTERBLOOM3DMOD}/WQP-WQP518MA.step
D_SMA | ${KICAD6_3DMODEL_DIR}/Diode_SMD.3dshapes/D_SMA.wrl
"#;
        assert_eq!(file, TEST_MD);
        fs::remove_file("test.md").expect("Must be able to delete test.md");
    }
}
