use std::borrow::Cow;
use std::env::args;
use std::fs::{File, read_dir, read_link, read_to_string};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use std::str::Lines;

#[derive(Clone)]
struct Atom<'a> {
    modifier: &'a str,
    package: String,
    version: String,
    slot: String
}

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
//used to collect all accepted licences
fn licence_walk(startpoint: &str, lines: Lines, mut licences: Vec<String>) -> Vec<String> {
    let cleanpoint = [startpoint.strip_prefix("@").unwrap(), " "].join("");
    for line in lines.clone() {
        if line.starts_with(cleanpoint.as_str()) {
            let line_split: Vec<&str> = line.split_whitespace().collect();
            for split in line_split {
                if split.starts_with('@') {
                    licences = licence_walk(split, lines.clone(), licences);
                } else {
                    licences.push(split.to_string());
                }
            }
        }
    }
    return licences;
}
//used to find all used profile directories
fn profile_walk(profile: PathBuf, mut profiles: Vec<Cow<str>>) -> Vec<Cow<str>> {
    let pclone = profile.clone();
    for path in read_dir(profile.clone()).unwrap() {
        let path_real = path.unwrap().path();
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
                let mut p_string = p_local.display().to_string();
                if p_string.chars().last().unwrap() != '/' {
                    p_string += "/";
                }
                let mut dupe = false;
                for p in profiles.clone() {
                    if p == p_string {
                        dupe = true;
                    }
                }
                if !dupe {
                    profiles.push(Cow::from(p_string));
                }
                profiles = profile_walk(p_local, profiles);
            }
        }
    }
    return profiles;
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
    let profiles = profile_walk(profile_start.parse().unwrap(), vec![profile_start]);
    //read accepted licences
    let licences = licence_walk("@FREE", read_to_string("/var/db/repos/gentoo/profiles/license_groups").unwrap().lines(), vec![]);
    //read profile data
    let mut a = "".to_string();
    let mut arch = "".to_string();
    let mut b = "".to_string();
    let mut d = "".to_string();
    let mut e = "".to_string();
    let mut f = "".to_string();
    let mut file_paths: Vec<PathBuf> = vec![];
    let mut use_expands: Vec<String> = vec![];
    let mut use_flags: Vec<String> = vec![];
    let mut world: Vec<Atom> = vec![];
    for profile in profiles {
        println!("{}", profile);
        for path in read_dir(profile.to_string()).unwrap() {
            let path_real = path.unwrap().path();
            let path_str = path_real.to_string_lossy().to_string();
            if path_str.contains("package.mask") {
                a = a.clone() + &*read_to_string(path_real.clone()).unwrap();
            } else if path_str.ends_with("/package.use") || path_str.ends_with("/package.use.force") {
                b = b.clone() + &*read_to_string(path_real.clone()).unwrap();
            } else if path_str.ends_with("packages") {
                for mut line in read_to_string(path_real.clone()).unwrap().lines() {
                    if !line.starts_with("#") && !line.is_empty() {
                        if line.starts_with("*") {
                            line = line.strip_prefix("*").unwrap();
                        }
                        let mut modifier = "";
                        let mut version = "";
                        let mut dupe = false;
                        if line.starts_with(">=") {
                            modifier = ">=";
                            version = line.split("-").last().unwrap();
                            line = &line[2..line.len() - (version.len() + 1)];
                            for mut p in world.clone() {
                                if p.package == line {
                                    if p.version.is_empty() {
                                        p.version = version.to_string()
                                    } else if p.version >= version.to_string() {
                                        dupe = true
                                    }
                                }
                            }
                        }
                        let mut line_str = line.to_string();
                        let mut slot = "";
                        if line.contains(":") {
                            slot = line.split(":").last().unwrap();
                            line_str = line.strip_suffix((":".to_string() + slot).as_str()).unwrap().to_string();
                        }
                        for p in world.clone() {
                            if p.package == line_str {
                                dupe = true;
                            }
                        }
                        if !dupe {
                            world.push(Atom { modifier, package: line_str, version: version.to_string(), slot: slot.to_string()});
                        }
                    }
                }
            } else if path_str.ends_with("/use.mask") || path_str.ends_with("/use.stable.mask") {
                d = d.clone() + &*read_to_string(path_real.clone()).unwrap();
            } else if path_str.ends_with("/package.use.mask") || path_str.ends_with("/package.use.stable.mask") {
                e = e.clone() + &*read_to_string(path_real.clone()).unwrap();
            } else if path_str.ends_with("/use.force") || path_str.ends_with("/use.stable.force") {
                f = f.clone() + &*read_to_string(path_real.clone()).unwrap();
            } else if path_str.ends_with("/make.defaults") {
                let lines = BufReader::new(File::open(path_real.clone()).unwrap()).lines();
                for line in lines {
                    let unline = line.unwrap();
                    if unline.starts_with("USE_EXPAND=\"") {
                        for split in unline[12..unline.len() - 1].split_whitespace() {
                            use_expands.push(split.to_string());
                        }
                    }
                }
                file_paths.push(path_real);
            }
        }
    }
    for file_path in file_paths {
        let lines = BufReader::new(File::open(file_path).unwrap()).lines();
        for line in lines {
            let unline = line.unwrap();
            for use_expand in &use_expands {
                if unline.starts_with(use_expand) {
                    for split in unline[use_expand.len() + 2..unline.len() - 1].split_whitespace() {
                        use_flags.push(use_expand.to_lowercase() + "_" + split);
                    }
                }
            }
            if unline.starts_with("USE") {
                for split in unline[5..unline.len() - 1].split_whitespace() {
                    if split != "${USE}" {
                        use_flags.push(split.to_string());
                    }
                }
            } else if unline.starts_with("ARCH") {
                arch = unline[6..unline.len() - 1].to_string();
            }
        }
    }
    let mut installed: Vec<Atom> = vec![];
    //get list of installed packages, could be made native by emulating ls /var/db/pkg/*
    let profile_cmd = Command::new("sh")
        .arg("-c")
        .arg("qlist -IUv")
        .output()
        .expect("failed to get installed packages");
    for mut line in String::from_utf8_lossy(&profile_cmd.stdout).lines() {
        line = line.split_whitespace().next().unwrap();
        let crumb = line.split("-").last().unwrap();
        let mut version: String;
        if crumb.starts_with("r") {
            let revision = crumb;
            version = line[..line.len() - (revision.len()+1)].split("-").last().unwrap().to_string() + "-" + revision;
        } else {
            version = crumb.to_string();
        }
        let package = line[..line.len() - (version.len() + 1)].to_string();
        let pkgdir = "/var/db/repos/gentoo/".to_string() + &*package;
        for path in read_dir(pkgdir.clone()).unwrap() {
            let path_real = path.unwrap().path();
            if path_real.ends_with("ebuild") {
                let pathstr = path_real.to_string_lossy();
                let newversion = &pathstr[pkgdir.len() + 1 + package.split("/").last().unwrap().len()..pathstr.len() - 7];
                if newversion > &version {
                    version = newversion.to_string();
                }
            }
        }
        installed.push(Atom { modifier: "", package, version, slot: "".to_string() });
    }
    println!("packages to be installed:");
    let mut empty = true;
    //check if world is installed
    for package in world {
        let mut dupe = false;
        for install in installed.clone() {
            if package.package == install.package {
                dupe = true
            }
        }
        if !dupe {
            empty = false;
            println!("{}", package.package)
        }
    }
    if empty {
        println!("nothing to install")
    }
}