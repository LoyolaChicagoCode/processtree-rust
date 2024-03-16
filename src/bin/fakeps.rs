extern crate rand;
use rand::{thread_rng, Rng};

extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of fake processes to generate
    #[arg(short, long, default_value_t = 10)]
    number: usize,
}

fn main() {
    let args = Args::parse();
    let n = args.number + 1;

    let mut ps = vec![vec![0usize; 0]; n];

    let mut rng = thread_rng();
    ps[0].push(1);
    for next_pid in 2usize..n {
        let random_pid: usize = 1 + rng.gen_range(0..next_pid - 1);
        ps[random_pid].push(next_pid);
    };

    println!("{:>10}{:>10} CMD", "PID", "PPID");
    for (ppid, children) in ps.iter().enumerate() {
        for pid in children {
            println!("{:>10}{:>10} Fake process {}", *pid, ppid, pid);
        }
    }
}

