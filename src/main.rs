mod path_reader;
mod path_printer;

use std::env;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut command_line_paths = env::args();
    command_line_paths.next(); // Pop program name off

    let paths = path_reader::read(command_line_paths, stdin.lock());
    path_printer::print(paths);
}

