extern crate rand;

use std::env;
use std::process;
use rand::{thread_rng, Rng};

fn help() {
    println!("usage: usage: fakeps n (where n > 0 = number of process table entries)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        help();
        process::exit(1);
    }

    let n: usize = match args[1].parse() {
        Ok(n) => n,
        _ => {
            help();
            process::exit(1);
        }
    };

    let mut ps = vec![vec![0usize; 0]; n];

    let mut rng = thread_rng();
    ps[0].push(1);
    for next_pid in 2usize..n {
        let random_pid: usize = 1 + rng.gen_range(0..next_pid - 1);
        ps[random_pid].push(next_pid);
    };

    println!("{:>10}{:>10} {}", "PID", "PPID", "CMD");
    for ppid in 0..n {
        for pid in &ps[ppid] {
            println!("{:>10}{:>10} Fake process {}", pid, ppid, pid);
        }
    }
}

