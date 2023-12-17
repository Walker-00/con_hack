use std::{fs, io::stdin, process::Command};

fn main() {
    println!("How Much?: ");
    let mut count = String::new();
    stdin().read_line(&mut count).unwrap();
    for _ in 0..count.trim().parse().unwrap() {
        fs::write("./d", b"b").unwrap();
        Command::new("git")
            .args(["add", "-A"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "\"hackk\""])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
