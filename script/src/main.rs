use std::{io::stdin, process::Command};

fn main() {
    println!("How Much?: ");
    let mut count = String::new();
    stdin().read_line(&mut count).unwrap();
    for _ in 0..count.trim().parse().unwrap() {
        Command::new("echo")
            .args(["\"b\"", ">>", "d"])
            .output()
            .unwrap();
        Command::new("git").args(["add", "-A"]).output().unwrap();
        Command::new("git")
            .args(["commit", "-m", "\"hackk\""])
            .output()
            .unwrap();
    }
}
