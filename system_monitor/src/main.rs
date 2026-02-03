use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("top")
        .args(["-b", "-n", "1", "-o", "%MEM"])
        .stdout(Stdio::piped())
        .spawn()     
        .expect("Error executing top");

    let top_five = Command::new("head")
        .args(["-n", "12"])
        .stdin(Stdio::from(output.stdout.unwrap()))
        .output()
        .expect("error executing head");

    let results = String::from_utf8_lossy(&top_five.stdout);

    println!("{}", results);
}
