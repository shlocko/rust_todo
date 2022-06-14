use crate::prelude::*;

pub fn parse_command(input: String) -> (Option<String>, Vec<String>) {
    let split = input.split(' ');
    let mut opt: Option<String> = None;
    let mut args: Vec<String> = Vec::new();
    for s in split {
        if opt == None {
            opt = Some(s.to_lowercase());
        } else {
            args.push(s.to_string());
        }
    }
    (opt, args)
}

pub fn run_command(input: String) {
    let (cmd, args) = parse_command(input);
    match cmd.as_deref() {
        Some("test") => {
            for s in args {
                println!("{}",s);
            }
        },
        _ => println!("Not a command")
    }
}
