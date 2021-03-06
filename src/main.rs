extern crate difference;
extern crate getopts;

use std::env;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("s", "split", "", "char|word|line");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let split = match matches.opt_str("s") {
        Some(ref x) if x == "char" => "",
        Some(ref x) if x == "word" => " ",
        Some(ref x) if x == "line" => "\n",
        _ => " "
    };

    if matches.free.len() > 1 {
        difference::print_diff(&matches.free[0], &matches.free[1], split);
    } else {
        print!("{}", opts.usage(&format!("Usage: {} [options]", program)));
        return;
    };


}

