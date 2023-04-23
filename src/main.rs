use std::{fs};
use std::env::args;
use std::process::Command;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 =>  prompt(false),
        2 => {
            if &args[1] == "update" {
                println!("updating system...");
                let contents = fs::read_to_string("/etc/portage/make.conf")
                    .expect("file read error");
                let system_set = Command::new("sh")
                    .arg("-c")
                    .arg("readlink -f /etc/portage/make.profile")
                    .output()
                    .expect("failed to execute process");
                println!("status: {}", system_set.status);
                println!("stdout: {}", String::from_utf8_lossy(&system_set.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&system_set.stderr));
                println!("{contents}")
            } else if &args[1] == "install" {
                panic!("please specify packages to install")
            } else {
                prompt(true)
            }
        },
        _ => {
            if &args[1] == "install" {
                println!("Installing {}...", &args[2])
            } else {
                prompt(true)
            }
        }
    }
}
fn prompt(u: bool) {
    const P: &str = ", please choose install, or update";
    if u {
        panic!("unknown command{}", P)
    } else {
        panic!("no commands specified{}", P)
    }
}