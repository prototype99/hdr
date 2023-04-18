use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/seirra/Documents/src/hdr/.gitignore")
        .expect("file read error");
    println!("{contents}");
}
