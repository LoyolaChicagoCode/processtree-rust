use std::env;
use std::process;

fn help() {
    println!("usage: usage: fakeps n (where n > 0 = number of process table entries)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        help();
        process::exit(1);
    }

    let n: i32 = match args[1].parse() {
        Ok(n) => n,
        _ => {
            help();
            process::exit(1);
        }
    };

    println!("n = {0}", n);
}

