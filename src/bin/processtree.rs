use std::collections::HashMap;
use std::io::{self, BufRead};

// for simplified error handling
extern crate anyhow;
use anyhow::{Context, Result};

use processtree::timestamps::{mark_time, print_timestamps};

type Process = (usize, usize, String);

type ProcessTree = HashMap<usize, Vec<Process>>;

// Option: when the absence of a value is part of the correct behavior
// Result: when actual errors are possible
// Conjecture: Result should never get projected to an Option

/// Creates a line parser from a header of the form "... PID ... PPID ... COMMAND ..." (or CMD).
fn make_parser_from_header(header: &str) -> Result<impl Fn(&str) -> Result<Process>> {
    let cols: Vec<&str> = header.split_whitespace().collect();
    let pos_in_cols = |col: &str| cols.iter().position(|&r| r == col);
    let i_pid = pos_in_cols("PID").context("No PID column")?;
    let i_ppid = pos_in_cols("PPID").context("No PPID column")?;
    let i_cmd = ["CMD", "COMMAND"].iter()
        .filter_map(|&cmd| header.find(cmd))
        .max()
        .context("No CMD or COMMAND column") // converts Option to Result
        ?; // unwraps Result and returns early if Err
    Ok(move |line: &str| {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let pid = tokens.get(i_pid).context("Input line too short")?.parse::<usize>()?;
        let ppid = tokens.get(i_ppid).context("Input line too short")?.parse::<usize>()?;
        let cmd = line.get(i_cmd..).context("Input line too short")?.trim();
        Ok((pid, ppid, cmd.to_string()))
    })
}

/// Builds a process tree from a flat list of processes.
fn build_tree(processes: Vec<Process>) -> ProcessTree {
    let mut tree: HashMap<usize, Vec<(usize, usize, String)>> = HashMap::new();
    for (pid, ppid, cmd) in processes {
        tree.entry(ppid).or_insert(Vec::with_capacity(5)).push((pid, ppid, cmd));
    }
    tree
}

/// Prints a process tree with indentation.
fn print_tree(tree: &ProcessTree, pid: usize, depth: usize) {
    if let Some(processes) = tree.get(&pid) {
        for (cid, _, cmd) in processes {
            println!("{:indent$}{}: {}", "", *cid, cmd, indent = depth * 2);
            print_tree(tree, *cid, depth + 1);
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    mark_time("Start time");
    let mut lines = io::stdin().lock().lines();
    let header = lines
        .next()
        .context("No header line") // converts Option to Result
        ? // unwraps outer Result and returns early if Err
        ?; // unwraps remaining inner Result
    mark_time("Setup time");
    let parser = make_parser_from_header(header.as_str()).context("Invalid header")?;
    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html
    let processes = lines
        .map(|line| parser(&line?)) // ? unwraps Result from each line
        .collect::<Result<_>>() // fails if parsing one line fails
        ?;
    mark_time("Input time");
    let tree = build_tree(processes);
    mark_time("Insertion time");
    print_tree(&tree, 0, 0);
    mark_time("Output time");
    println!("Total processes: {}", tree.len());
    print_timestamps();
    Ok(())
}
