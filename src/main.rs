use ministat::*;
use std::env;

fn main() {
    // Read in file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./ministat filename.txt");
        std::process::exit(1);
    }
    let filename = &args[1];
    let ds = readset_mt(filename);
    ds.vitals();
}
