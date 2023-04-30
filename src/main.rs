use std::borrow::Cow;
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
fn profile_walk(profile: PathBuf, profiles: Vec<Cow<str>>){
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
                let p_string = p_local.to_string_lossy();
                if p_string.chars().last().unwrap() != '/' {
                    println!("{}",p_string + "/");
                } else {
                    println!("{}",p_string);
                }
                profile_walk(p_local, profiles.clone());
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
    let profile_start = profile_result.to_string_lossy();
    println!("{}", profile_start);
    let profiles = vec![profile_start.clone()];
    profile_walk(profile_start.parse().unwrap(), profiles.clone());
    //generate profile data
    let mut a: String = "".to_string();
    let mut b: String = "".to_string();
    let mut c: String = "".to_string();
    let mut d: String = "".to_string();
    let mut e: String = "".to_string();
    let mut f: String = "".to_string();
    for profile in profiles {
        let path_real = PathBuf::from(profile.to_string());
        let path_str = path_real.to_string_lossy();
        if path_str.contains("package.mask") {
            a = a.clone() + &*read_to_string(path_real).unwrap();
        } else if path_real.ends_with("/package.use") || path_real.ends_with("/package.use.force") {
            b = b.clone() + &*read_to_string(path_real).unwrap();
        } else if path_str.contains("packages") {
            c = c.clone() + &*read_to_string(path_real).unwrap();
        } else if path_real.ends_with("/use.mask") || path_real.ends_with("/use.stable.mask") {
            d = d.clone() + &*read_to_string(path_real).unwrap();
        } else if path_real.ends_with("/package.use.mask") || path_real.ends_with("/package.use.stable.mask") {
            e = e.clone() + &*read_to_string(path_real).unwrap();
        } else if path_real.ends_with("/use.force") || path_real.ends_with("/use.stable.force") {
            f = f.clone() + &*read_to_string(path_real).unwrap();
        }
    }
}