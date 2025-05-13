mod scanner;
use std::env;
use std::fs;
fn main() {
    let script = "[arg1] | None";
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: mal {}", script)
    } else if args.len() == 2 {
        run_file(&args[2]);
    } else {
        run_promt();
    }
}
fn run_file(path: &String) {
    if let Ok(file_contents) = fs::read_to_string(path) {
        run(&file_contents);
    } else {
        println!("File not found");
    };
}
fn run(file: &String) {}
