use std::{env, fs};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let modi = &args[2];
    if mode == "update" {
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
        println!("{contents}");
    } else if mode == "install" {
        println!("Installing {}...", modi);
    }
}
