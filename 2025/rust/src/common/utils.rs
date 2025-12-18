use std::fs::File;
use std::io::Read;

pub fn read_input(path: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(path).expect("Should be able to open file");
    f.read_to_string(&mut data)
        .expect("Should be able to read string");
    return data;
}

pub fn str_to_str_list<'a>(string: &'a str, sep: &str) -> Vec<&'a str> {
    return string.split(sep).filter(|x| !x.is_empty()).collect();
}
