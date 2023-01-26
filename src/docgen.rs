use lexpr::Value;

pub trait DocItem {
    fn elem(&self, el: &String) -> String;
}

pub fn find(val: &Value, root: &str, qualifier: &str, offset: usize) -> String {
    let mut root_idx = 0;
    loop {
        if val[root_idx].is_nil() {
            break;
        }
        if val[root_idx][0].to_string() == root
            && val[root_idx][1].to_string() == format!("\"{}\"", qualifier)
        {
            return val[root_idx][offset].to_string();
        }
        root_idx += 1;
    }
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use lexpr::parse::Error;

    const S_EXPRESSION: &'static str = r#"((dummy . "nothing")
        (name "first" "John")
        (age . 43)
        (phones "+44 1234567" "+44 2345678"))"#;

    #[test]
    fn find_can_traverse() -> Result<(), Error> {
        let v = lexpr::from_str(S_EXPRESSION)?;
        let name = crate::docgen::find(&v, "name", "first", 2);
        assert_eq!(name, "\"John\"");
        Ok(())
    }

    #[test]
    fn find_fail_traverse_with_blank() -> Result<(), Error> {
        let v = lexpr::from_str(S_EXPRESSION)?;
        let name = crate::docgen::find(&v, "nothing", "first", 2);
        assert_eq!(name, "");
        Ok(())
    }
}
