use std::collections::HashMap;
use std::time::SystemTime;
use std::io::{self, BufRead};

extern crate log;
use log::info;

// for simplified error handling
extern crate anyhow;
use anyhow::Result;

type Process = (usize, usize, String);

type ProcessTree = HashMap<usize, Vec<Process>>;

/// Creates a line parser from a header of the form "... PID ... PPID ... COMMAND ..." (or CMD).
fn make_parser_from_header(header: &str) -> Option<impl Fn(&str) -> Result<Process>> {
    let cols: Vec<&str> = header.split_whitespace().collect();
    let pos_in_cols = |col: &str| cols.iter().position(|&r| r == col).unwrap();
    let [i_pid, i_ppid] = ["PID", "PPID"].map(pos_in_cols);
    let i_cmd = ["CMD", "COMMAND"].iter()
        .filter_map(|&cmd| header.find(cmd))
        .max()?;
    Some(move |line: &str| { 
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let pid = tokens[i_pid].parse::<usize>()?;
        let ppid = tokens[i_ppid].parse::<usize>()?;
        let cmd = line[i_cmd..].trim();
        Ok((pid, ppid, cmd.to_string()))
    })
}

/// Builds a process tree from a flat list of processes.
fn build_tree(processes: impl Iterator<Item = Process>) -> ProcessTree {
    let mut tree: HashMap<usize, Vec<(usize, usize, String)>> = HashMap::new();
    for (pid, ppid, cmd) in processes {
        tree.entry(ppid).or_default().push((pid, ppid, cmd));
    }
    tree
}

/// Prints a process tree with indentation.
fn print_tree(tree: &ProcessTree, pid: usize, depth: usize) {
    if let Some(processes) = tree.get(&pid) {
        for (pid, _, cmd) in processes {
            println!("{:indent$}{}: {}", "", pid, cmd, indent = depth * 2);
            print_tree(tree, *pid, depth + 1);
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let mut lines = io::stdin().lock().lines();
    let header = lines.next().unwrap()?;
    let parser = make_parser_from_header(header.as_str()).unwrap();
    let processes = lines.map(|line: Result<String, io::Error>| line.unwrap()).map(|line| parser(&line).unwrap());
    let start = SystemTime::now();
    let tree = build_tree(processes);
    let total = SystemTime::now().duration_since(start)?;
    print_tree(&tree, 0, 0);
    info!("Processing time: {:?}", total);
    Ok(())
}
