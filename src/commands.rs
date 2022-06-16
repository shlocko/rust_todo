use crate::prelude::*;
use std::collections::HashMap;

// Takes in the string from user input, 
// splits it into a command (Option) and args (Vector of Strings)

pub fn parse_input(input: String) -> Option<(String, Vec<String>)> {
    let split = input.split(' ');
    let mut cmd: String = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut opt: Option<(String, Vec<String>)> = None;
    
    if !input.is_empty() {
        for s in split {
            if cmd.is_empty() {
                cmd = s.to_lowercase();
            } else {
                args.push(s.to_string());
            }
        }
        opt = Some((cmd, args));
    }
    opt
}

// Send user input to parse_command(), then matches the command Option
// to execute the associated function
pub fn run_command(input: String) {

    // Create a HashMap of commands
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();

    // Fill that HashMap of commands
    commands.insert(
        "test".to_string(),
        |args: Vec<String>| {
            for s in args {
                println!("{}", s);
            }
        }
        );
     
    let parsed_command = parse_input(input); // Prarse the input

    match parsed_command {
        Some((cmd, args)) => { // Checks that there was input
            let func = commands.get(&cmd); // Pulls the command funciton out of the HashMap
            match func {
                Some(func) => func(args), // If found, execute the command
                None => println!("{} is not a valid command", cmd)
            }
        },
        None => println!("You did not input a command")
    }

}
