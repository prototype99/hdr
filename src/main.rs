use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::process::Command;

fn main() {
    //collect arguments
    let args: Vec<String> = args().collect();
    //number of arguments influences interactions
    match args.len() {
        1 =>  prompt(false),
        2 => {
            if &args[1] == "update" {
                update()
            } else if &args[1] == "install" {
                panic!("please specify packages to install")
            } else {
                prompt(true)
            }
        },
        _ => {
            if &args[1] == "install" {
                println!("Installing {}...", &args[2])
            } else if &args[1] == "update" {
                update()
            } else {
                prompt(true)
            }
        }
    }
}
//used to gain all necessary profile information
fn profile_walk(p: String){
    let paths = read_dir(p.clone()).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    let prefix = "ls -p ".to_string();
    let suffix = "| grep -v /".to_string();
    let pclone = p.clone();
    let list_cmd = Command::new("sh")
        .arg("-c")
        .arg([prefix, p, suffix].join(""))
        .output()
        .expect("failed to list files");
    for line in String::from_utf8_lossy(&list_cmd.stdout).to_string().lines() {
        println!("{}", line);
        if line == "parent" {
            let parent_suffix: String = "/parent".to_string();
            let contents = read_to_string([pclone.clone(),parent_suffix].join(""))
                .expect("file read error");
            println!("{contents}");
        }
    }
}
//this function helps deduplicate strings
fn prompt(u: bool) {
    const P: &str = ", please choose install, or update";
    if u {
        panic!("unknown command{}", P)
    } else {
        panic!("no commands specified{}", P)
    }
}
fn update() {
    println!("updating system...");
    //find the starting point for profile, is there a native rust function?
    let profile_cmd = Command::new("sh")
        .arg("-c")
        .arg("readlink -f /etc/portage/make.profile")
        .output()
        .expect("failed to determine profile");
    //println!("status: {}", system_set.status);
    let profile = String::from_utf8_lossy(&profile_cmd.stdout).trim().to_string();
    println!("{}",profile);
    profile_walk(profile);
    //println!("stderr: {}", String::from_utf8_lossy(&system_set.stderr));
}