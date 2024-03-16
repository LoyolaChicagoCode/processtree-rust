use std::{collections::HashMap, io::{self, BufRead}, num::ParseIntError, time::SystemTime};

type Process = (usize, usize, String);

type ProcessTree = HashMap<usize, Vec<Process>>;

fn parse_header(header: &str) -> Option<impl Fn(&str) -> Result<Process, ParseIntError>> {
    let cols: Vec<&str> = header.trim().split_whitespace().collect();
    let i_pid = cols.iter().position(|&r| r == "PID")?;
    let i_ppid = cols.iter().position(|&r| r == "PPID")?;
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

fn build_tree(processes: impl Iterator<Item = Process>) -> ProcessTree {
    processes.fold(HashMap::new(), |mut tree, process| {
        let (pid, ppid, cmd) = process;
        tree.entry(ppid).or_default().push((pid, ppid, cmd));
        tree
    })  
}

fn print_tree(tree: &ProcessTree, pid: usize, depth: usize) {
    if let Some(processes) = tree.get(&pid) {
        for (pid, _, cmd) in processes {
            println!("{:indent$}{}: {}", "", pid, cmd, indent = depth * 2);
            print_tree(tree, *pid, depth + 1);
        }
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let header = lines.next().unwrap().unwrap();
    let parser = parse_header(header.as_str()).unwrap();
    let processes = lines.map(|line| line.unwrap()).map(|line| parser(&line).unwrap());
    let start = SystemTime::now();
    let tree = build_tree(processes);
    let total = SystemTime::now().duration_since(start).unwrap();
    print_tree(&tree, 0, 0);
    println!("Processing time: {:?}", total);
}
