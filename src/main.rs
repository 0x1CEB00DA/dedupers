#![allow(dead_code)]
extern crate dedupers;
use dedupers::scan::scan;


fn main() {
    let scan_result = scan(&["/etc"]);
    // match scan_result {
    //     Ok(files) => 
    //         for file in files {
    //             println!("{:?}", file);
    //         },
    //     Err(err) => println!("ERROR: {}", err),
    // }
    let (files, errors) = scan_result;
    for file in files {
        println!("{:?}", file);
    }
    println!("ERRORS:");
    for error in errors {
        println!("{:?}", error);
    }

}
