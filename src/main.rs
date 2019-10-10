use std::{env, process};

mod rom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} [path to rom file]", args[0]);
        process::exit(1);
    }

    let rom_path: &str = &args[1];
    println!("Loading rom at path: {}", rom_path);

    let rom = rom::Rom::load(rom_path);
}
