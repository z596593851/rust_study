
use std::env;
use std::process;
use demo::Config;

fn main() {
    // C:\Users\59659\Desktop\my_file.txt
    // io_demo::io::input();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = demo::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}


