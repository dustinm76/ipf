use std::env;
use std::io;
use std::io::Read;

pub fn parse(args: env::Args, mut stdin: io::StdinLock) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    for argument in args {
        paths.push(argument);
    }

    if paths.is_empty() {
        let mut bytes: Vec<u8> = Vec::new();
        let _ = stdin.read_to_end(&mut bytes);

        let input = String::from_utf8(bytes).unwrap();
        let lines = input.split("\n"); //TODO if null-input, split on \0

        for line in lines {
            paths.push(line.to_string());
        }
    }

    return paths;
}
