use std::env::args;
use std::fs::read_dir;
use std::process::Command;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 =>  prompt(false),
        2 => {
            if &args[1] == "update" {
                println!("updating system...");
                //let contents = fs::read_to_string("/etc/portage/make.conf")
                //    .expect("file read error");
                let system_set = Command::new("sh")
                    .arg("-c")
                    .arg("readlink -f /etc/portage/make.profile")
                    .output()
                    .expect("failed to determine profile");
                //println!("status: {}", system_set.status);
                let profile = String::from_utf8_lossy(&system_set.stdout).to_string();
                println!("{}",profile);
                //let paths = read_dir(profile).unwrap();
                let profile = String::from_utf8_lossy(&system_set.stdout).to_string();
                let paths = match read_dir(profile) {
                    Ok(p) => p,
                    Err(_) => {
                        let prefix = "/mnt/gentoo".to_string();
                        let profile = String::from_utf8_lossy(&system_set.stdout).to_string();
                        println!("{}", [prefix, profile].join(""));
                        let prefix = "/mnt/gentoo".to_string();
                        let profile = String::from_utf8_lossy(&system_set.stdout).to_string();
                        match read_dir([prefix, profile].join("")) {
                            Ok(q) => q,
                            Err(e) => !panic!("{}", e)
                        }
                    }
                };
                for path in paths {
                    println!("Name: {}", path.unwrap().path().display())
                }
                //println!("stderr: {}", String::from_utf8_lossy(&system_set.stderr));
                //println!("{contents}")
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