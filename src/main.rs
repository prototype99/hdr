use std::env::args;
use std::fs::{read_dir, read_link, read_to_string};
use std::path::PathBuf;

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
fn profile_walk(profile: PathBuf, mut d: [String; 4]){
    let pclone = profile.clone();
    let paths = read_dir(profile).unwrap();
    for path in paths {
        let path_unwrap = path.unwrap();
        let path_real = path_unwrap.path();
        let path_str = path_real.to_string_lossy();
        //check for parent directories
        if path_str.contains("parent") {
            for line in read_to_string(path_real).unwrap().lines() {
                let mut line_path = PathBuf::from(line);
                let mut p_local = pclone.clone();
                while line_path.starts_with("..") {
                    p_local.pop();
                    line_path = PathBuf::from(line_path.strip_prefix("../").expect("error calculating profile parent"));
                }
                p_local.push(line_path);
                println!("{}",p_local.to_string_lossy());
                profile_walk(p_local, d.clone());
            }
        } else if path_str.contains("package.mask") {
            d[0] = d[0].clone() + &*read_to_string(path_unwrap.path()).unwrap();
        } else if path_str.contains("package.use") {
            d[1] = d[1].clone() + &*read_to_string(path_unwrap.path()).unwrap();
        } else if path_str.contains("packages") {
            d[2] = d[2].clone() + &*read_to_string(path_unwrap.path()).unwrap();
        } else if path_real.ends_with("/use.mask") {
            d[3] = d[3].clone() + &*read_to_string(path_unwrap.path()).unwrap();
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
    let a: String = "".to_string();
    let b: String = "".to_string();
    let c: String = "".to_string();
    let d: String = "".to_string();
    profile_walk(PathBuf::from(profile.to_string()), [a, b, c, d]);
}