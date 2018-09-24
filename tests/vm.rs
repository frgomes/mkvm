use std::env;
use mkvm::yaml;

extern crate mkvm;

#[test]
fn parse_virtual_machine() {
    let input = format!("{}/tests/input01.yaml", env::current_dir().unwrap().display());
    let files: Vec<&str> = vec![input.as_str()];
    yaml::files(&files);
    assert!(true);
}
