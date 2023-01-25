use std::io::Write;

use crate::docgen::DocItem;

pub fn table_sep(w: &mut dyn std::io::Write, format: &str) -> Result<(), std::io::Error> {
    writeln!(w, "")?;
    let mut first = true;
    for _ in format.split("|") {
        if !first {
            write!(w, "|")?;
        }
        write!(w, "---")?;
        first = false;
    }
    Ok(())
}

pub(crate) fn table_header(w: &mut std::fs::File, format: &str) -> Result<(), std::io::Error> {
    let mut first = true;
    for elem in format.split("|") {
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

pub(crate) fn title(w: &mut std::fs::File, title: &str) -> Result<(), std::io::Error> {
    write!(w, "# {}\n\n", title)?;
    Ok(())
}

pub(crate) fn table_content<T: DocItem>(
    w: &mut std::fs::File,
    format: &str,
    docs: &mut Vec<T>,
) -> Result<(), std::io::Error> {
    writeln!(w, "")?;
    for doc in docs {
        let mut first = true;
        for elem in format.split("|") {
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
