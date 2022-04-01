use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let source = read_to_string("../../ReadMe.md").unwrap();

    let mut files = HashMap::new();

    files.insert("ReadMe", source.clone());
}
