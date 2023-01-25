use lexpr::Value;

pub trait DocItem {
    fn elem(&self, el: &str) -> String;
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
