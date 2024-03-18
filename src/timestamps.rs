// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

extern crate log;
use self::log::info;

type TimeStamp = (String, SystemTime);

fn timestamps() -> &'static Mutex<Vec<TimeStamp>> {
    static ARRAY: OnceLock<Mutex<Vec<TimeStamp>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

pub fn mark_time(label: &str) {
    timestamps().lock().unwrap().push((label.to_string(), SystemTime::now()));
}

pub fn print_timestamps() {
    for window in timestamps().lock().unwrap().windows(2) {
        if let [(_, t0), (label, t1)] = window {
            let time = t1.duration_since(*t0).unwrap();
            info!("{}: {:?}", label, time);    
        }
    }
    let start = timestamps().lock().unwrap().first().unwrap().1;
    let stop = timestamps().lock().unwrap().last().unwrap().1;
    info!("TOTAL time: {:?}", stop.duration_since(start).unwrap());
}
