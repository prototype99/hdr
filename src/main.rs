use std::{fs};
use std::env::args;
use std::process::Command;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 =>  panic!("no commands specified, please choose install, or update"),
        2 => {
            if &args[1] == "update" {
                println!("updating system...");
                let contents = fs::read_to_string("/etc/portage/make.conf")
                    .expect("file read error");
                let system_set = Command::new("sh")
                    .arg("-c")
                    .arg("eix-update && eix -c --system")
                    .output()
                    .expect("failed to execute process");
                println!("status: {}", system_set.status);
                println!("stdout: {}", String::from_utf8_lossy(&system_set.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&system_set.stderr));
                println!("{contents}")
            } else if &args[1] == "install" {
                panic!("please specify packages to install")
            } else {
                panic!("unknown command, please choose install, or update")
            }
        },
        _ => {
            if &args[1] == "install" {
                println!("Installing {}...", &args[2]);
            } else {
                panic!("unknown command, please choose install, or update")
            }
        }
    }
}
