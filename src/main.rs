mod scanlib;
use crate::scanlib::scan_command::scan_command;
use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();

    //dbg!(args.iter());
    let file=&args[1];
    let data = scan_command(file);
    dbg!(data);
}
