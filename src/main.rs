use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, stdin, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use chrono;

fn main() {
    backup();
}

/// this function is to run command and print output
fn run(command: String) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture stdout."))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture stdout."))
    };

    let reader = BufReader::new(output.unwrap());

    reader.lines().for_each(|line| println!("{}", line.unwrap()));
}

/// backup folder
fn backup() {
    // get current date
    let current_date = chrono::offset::Local::now().date_naive().to_string();

    // ask for folder path
    println!("target directory path:");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();


    // trim folder path
    let mut path = PathBuf::new();
    path.push(&input.trim().trim_matches('"'));

    // if is folder
    if path.is_dir() {

        // go to parent folder
        let dir = path.parent().unwrap();
        let target_folder = path.file_name().unwrap().to_os_string().into_string().unwrap();

        // check if the target folder name contains space
        if target_folder.contains(" ") {
            println!("Folder name contains space.");
            return;
        };

        // ask for backup method
        println!("1 - full");
        println!("2 - diff");
        println!("choose backup method: ");
        input.clear();
        stdin().read_line(&mut input).unwrap();


        let mut password = String::new();
        if !Path::new("config.toml").exists() {
            // ask for password
            println!("password for the .rar file: ");
            stdin().read_line(&mut password).unwrap();
            // save password to config.toml where the program at
            save_password(&password).expect("Save config failed.");
        } else {
            password = read_password();
        }

        let mut command = String::from("rar a -ep1 -rr5p -m5 -md512 -hp") + &password.trim() + " -ac ";
        let mut output_path = String::new();

        if input.trim() == "1" {
            // 1 - full backup
            output_path.push_str(&target_folder);
            output_path.push_str("-");
            output_path.push_str(&current_date);
            output_path.push_str(".rar");
            output_path.push_str(" ");
        } else if input.trim() == "2" {
            // 2 - diff backup
            command.push_str("-ao ");

            output_path.push_str(&target_folder);
            output_path.push_str("-");
            output_path.push_str(&current_date);
            output_path.push_str(".rar");
            output_path.push_str(" ");
        } else {
            println!("Doing nothing...");
            return;
        }

        command = command + &output_path + &target_folder;

        // go to target folder
        assert!(env::set_current_dir(&dir).is_ok());

        println!("{}", command);
        run(command);
    } else {
        println!("input should be a directory path");
    }
}

/// save config using toml
fn save_password(password: &str) -> std::io::Result<()> {
    // create file
    let mut file = File::create("config.toml")?;
    file.write_all(password.as_ref())?;
    Ok(())
}

/// read config
fn read_password() -> String {
    let mut file = File::open("config.toml").expect("Open file failed.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Read file failed.");

    return contents;
}