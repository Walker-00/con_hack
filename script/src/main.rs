use std::process::Command;

fn main() {
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
