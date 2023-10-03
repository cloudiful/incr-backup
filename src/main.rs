use chrono;
use std::process::Command;

fn main() {
    let current_date = chrono::offset::Local::now().date_naive();

    run("dir")
}

/// this function is to run command and print output
fn run(command: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };

    let output_str = String::from_utf8(output.stdout).expect("read output error");

    println!("{}", output_str);
}