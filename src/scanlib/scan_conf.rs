use std::fs;
//use std::path::Path;

pub fn scan_conf(file_path: &str) -> Option<Vec<String>> {
    //判断文件是否存在
    //if !Path::new(file_path).exists() {
    //    println!("file is not exists.");
    //}
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read input")
        .trim()
        .split("\n")
        .map(|s| s.to_string()) // Convert &str to String
        .collect();
    return Some(contents);
}
