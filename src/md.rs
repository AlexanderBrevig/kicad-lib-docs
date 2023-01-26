use crate::docgen::DocItem;

pub(crate) fn title(w: &mut dyn std::io::Write, title: &str) -> Result<(), std::io::Error> {
    write!(w, "# {}\n\n", title)?;
    Ok(())
}

pub(crate) fn table_header(
    w: &mut dyn std::io::Write,
    format: &Vec<String>,
) -> Result<(), std::io::Error> {
    let mut first = true;
    for elem in format {
        if !first {
            write!(w, " | ")?;
        }
        write!(
            w,
            "{}{}",
            (&elem[..1].to_string()).to_uppercase(),
            &elem[1..]
        )?;
        first = false;
    }
    Ok(())
}

pub fn table_sep(w: &mut dyn std::io::Write, format: &Vec<String>) -> Result<(), std::io::Error> {
    writeln!(w, "")?;
    let mut first = true;
    for _ in format {
        if !first {
            write!(w, "|")?;
        }
        write!(w, "---")?;
        first = false;
    }
    Ok(())
}

pub(crate) fn table_content<T: DocItem>(
    w: &mut dyn std::io::Write,
    format: &Vec<String>,
    docs: &mut Vec<T>,
) -> Result<(), std::io::Error> {
    writeln!(w, "")?;
    for doc in docs {
        let mut first = true;
        for elem in format {
            if !first {
                write!(w, " | ")?;
            }
            write!(w, "{}", doc.elem(elem))?;
            first = false;
        }
        writeln!(w, "")?;
    }
    Ok(())
}

pub(crate) fn lexpr_str_to_md(s: String) -> String {
    return s.replace("#nil", "").replace("\"", "");
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Error};

    use crate::symbols::SymbolDoc;

    #[test]
    fn title() -> Result<(), Error> {
        let mut s = String::new();
        unsafe {
            let mut buffer = BufWriter::new(s.as_mut_vec());
            crate::md::title(&mut buffer, "My Title")?;
        }
        assert_eq!(s, "# My Title\n\n");
        Ok(())
    }

    #[test]
    fn table_header() -> Result<(), Error> {
        let mut s = String::new();
        let format: Vec<String> = vec!["one".to_string(), "two".to_string()];
        unsafe {
            let mut buffer = BufWriter::new(s.as_mut_vec());
            crate::md::table_header(&mut buffer, &format)?;
        }
        assert_eq!(s, "One | Two");
        Ok(())
    }

    #[test]
    fn table_sep() -> Result<(), Error> {
        let mut s = String::new();
        let format: Vec<String> = vec!["one".to_string(), "two".to_string()];
        unsafe {
            let mut buffer = BufWriter::new(s.as_mut_vec());
            crate::md::table_sep(&mut buffer, &format)?;
        }
        assert_eq!(s, "\n---|---");
        Ok(())
    }

    #[test]
    fn table_content() -> Result<(), Error> {
        let mut s = String::new();
        let mut docs = vec![
            SymbolDoc {
                symbol: "a".to_string(),
                reference: "b".to_string(),
                value: "c".to_string(),
                footprint: "d".to_string(),
                datasheet: "e".to_string(),
            },
            SymbolDoc {
                symbol: "A".to_string(),
                reference: "B".to_string(),
                value: "C".to_string(),
                footprint: "D".to_string(),
                datasheet: "E".to_string(),
            },
        ];
        let format: Vec<String> = vec!["symbol".to_string(), "value".to_string()];
        unsafe {
            let mut buffer = BufWriter::new(s.as_mut_vec());
            crate::md::table_content(&mut buffer, &format, &mut docs)?;
        }
        assert_eq!(s, "\na | c\nA | C\n");
        Ok(())
    }

    #[test]
    fn lexpr_str_to_md() {
        let s = crate::md::lexpr_str_to_md("\"remove quotes\"".to_string());
        assert_eq!(s, "remove quotes");

        let s = crate::md::lexpr_str_to_md("#nil".to_string());
        assert_eq!(s, "");
    }
}
