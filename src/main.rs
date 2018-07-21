mod options;
mod path_parser;

use std::env;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut command_line_paths = env::args();
    command_line_paths.next(); // Pop program name off

    let paths = path_parser::parse(command_line_paths, stdin.lock());

    for path in paths {
        println!("{}", path);
    }
}

