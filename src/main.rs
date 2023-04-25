use std::borrow::Cow;
use std::env::args;
use std::fs::{read_dir, read_link, read_to_string};

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
fn profile_walk(p: Cow<str>){
    let paths = read_dir(p.to_string()).unwrap();
    for path in paths {
        let path_unwrap = path.unwrap();
        println!("{}", path_unwrap.path().display());
        //check for parent directories
        if path_unwrap.path().to_string_lossy().contains("parent") {
            for line in read_to_string(path_unwrap.path()) {
                println!("{}", line);
            }
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
    //find the starting point for profile
    let profile_result = read_link("/etc/portage/make.profile").unwrap();
    let profile = profile_result.to_string_lossy();
    println!("{}",profile);
    //generate profile data
    profile_walk(profile);
}