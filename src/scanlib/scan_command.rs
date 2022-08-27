use crate::scanlib::scan_conf;
use crate::scanlib::scan_port;
//use crate::scanlib::scan_response;
//use std::process::exit;
use url::Url;
//use std::sync::mpsc;

#[derive(Copy, Clone)]
struct Addr {
    protocol: &'static str,
    protocol_args: &'static str,
    url_port: u16,
    url_port_head: &'static str,
}

pub fn scan_command(file: &str) -> Vec<String> {
    let con = scan_conf::scan_conf(file);
    let domain = Addr {
        protocol: "https",
        protocol_args: "://",
        url_port: 443u16,
        url_port_head: ":",
    };
    let mut data = vec![];
    //let mut url_args_tail= "/404.html".to_string();
    //let mut addr= protocol + &protocol_args + &url_values + &url_port_head + &(url_port.to_string()) + &url_args_tail;
    if let Some(vs) = con {
        std::thread::scope(|s| {
            //dbg!(&vs);
            let mut num = 0u32;
            let wait_millis = std::time::Duration::from_millis(1000);
            for v in vs {
                //let (tx, rx) = mpsc::channel();
                let vp = "".to_string() + domain.protocol + domain.protocol_args + &v;
                //let url = Url::parse(&vp).unwrap();
                let url = Url::parse(&vp).expect(&vp);
                let url_values = url.domain().unwrap().to_string();
                let url_args_tail = url.path();
                let mut addr = "".to_string()
                    + domain.protocol
                    + domain.protocol_args
                    + &url_values
                    + domain.url_port_head
                    + &(domain.url_port.to_string())
                    + url_args_tail;
                num += 1;
                if num % 5 == 0 {
                    //println!("sleep 1s.");
                    std::thread::sleep(wait_millis);
                }
                let data_values = s
                    .spawn(move || {
                        //println!("thread started.");
                        //get timeout
                        //https://stackoverflow.com/questions/36181719/what-is-the-correct-way-in-rust-to-create-a-timeout-for-a-thread-or-a-function
                        let is_alive = scan_port::scan_port(&url_values, &domain.url_port);
                        //dbg!(is_alive);
                        if is_alive == false {
                            //print!("{} check down.\n", &vp);
                            return vp;
                            //std::process::exit(0);
                        } else {
                            //get fail code
                            //https://docs.rs/curl/0.4.8/curl/easy/struct.Easy2.html
                            // let is_ok = crate::scanlib::scan_response::scan_response(&vp);
                            // if is_ok == false {
                            //    print!("{} response greater than or equal to 400.\n", &vp);
                            //   }
                            return "".to_string();
                        }
                    })
                    .join()
                    .ok()
                    .unwrap();
                if data_values.len() > 1 {
                    data.push(data_values);
                }
            }
        });
    }
    return data;
}
