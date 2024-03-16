extern crate assert_cmd;
extern crate predicates;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn fakeps_runs_without_args() {
    let mut cmd = Command::cargo_bin("fakeps").unwrap();
    cmd.assert()
    .success()
    .stdout(predicate::str::contains("PID"))
    .stdout(predicate::str::contains("Fake process"));
}
