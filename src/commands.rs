use crate::prelude::*;

// Takes in the string from user input, 
// splits it into a command (Option) and args (Vector of Strings)
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

// Send user input to parse_command(), then matches the command Option
// to execute the associated function
pub fn run_command(input: String) {
    let (cmd, args) = parse_command(input);
    match cmd.as_deref() {
        Some("test") => { // Test command, outputs args for testing
            for s in args {
                println!("{}",s);
            }
        },
        _ => println!("Not a command")
    }
}
