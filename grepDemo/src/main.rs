use grepDemo::Config;
use std::env;
use std::process;
fn main() {
    let args:Vec<String>= env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments:{}",err);
        process::exit(1);
    });
    if let Err(e) = grepDemo::run(conf) {
        eprintln!("Application error:{}",e);
        process::exit(1);
    }
 
}
