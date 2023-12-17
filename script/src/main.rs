use std::{
    fs,
    io::{stdin, Write},
    process::{Command, Stdio},
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
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "\"hackk\""])
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
