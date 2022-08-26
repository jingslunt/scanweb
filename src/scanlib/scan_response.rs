use curl::easy::{Easy,List};

pub fn scan_response(host: &str) -> bool {
    let mut handle = Easy::new();
    handle.url(host).unwrap();
    handle.perform().unwrap();
    //assert_eq!(easy.response_code().unwrap(), 400);
    if handle.response_code().unwrap() >= 400 {
        return false;
    } else {
        return true;
    };
}
