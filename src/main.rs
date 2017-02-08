#![allow(dead_code)]
extern crate dedupers;
use dedupers::scan::scan;


fn main() {
    let scan_result = scan(&["/tmp"]);
    match scan_result {
        Ok(files) => 
            for file in files {
                println!("{:?}", file);
            },
        Err(err) => println!("ERROR: {}", err),
    }
}
