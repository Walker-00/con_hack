use std::{
    fs,
    io::{stdin, Write},
    process::Command,
};

fn main() {
    println!("How Much?: ");
    let mut count = String::new();
    stdin().read_line(&mut count).unwrap();
    let mut file = fs::OpenOptions::new().append(true).open("./d").unwrap();
    for _ in 0..count.trim().parse().unwrap() {
        file.write_all(b"b").unwrap();
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
