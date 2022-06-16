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
        if args.is_empty(){
            args.push("".to_string());
        }
        opt = Some((cmd, args));
    }
    opt
}

// Send user input to parse_input(), then matches the command Option
// to execute the associated function
pub fn run_command(items: &mut Items, input: String) {

    // Create a HashMap of commands
    let mut commands: HashMap<String, fn(&mut Items, Vec<String>)> = HashMap::new();

    // Fill that HashMap of commands
    commands.insert(
        "test".to_string(),
        |_items: &mut Items, args: Vec<String>| {
            for s in args {
                println!("{}", s);
            }
        }
        );
     
    commands.insert(
        "add".to_string(),
        |items: &mut Items, args: Vec<String>| {
            let mut content = String::new();
            for s in args {
                content.push_str(&s);
            }
            items.add_entry(
                content
                );
        }
        );

    commands.insert(
        "list".to_string(),
        |items: &mut Items, _args: Vec<String>| {
            for i in 0..items.get_entries().len(){
                println!("{}", items.get_entries()[i].get_content());
            }
        }
        );
    
    let parsed_command = parse_input(input); // Prarse the input

    match parsed_command {
        Some((cmd, args)) => { // Checks that there was input
            let func = commands.get(&cmd); // Pulls the command funciton out of the HashMap
            match func {
                Some(func) => func(items, args), // If found, execute the command
                None => println!("{} is not a valid command", cmd)
            }
        },
        None => println!("You did not input a command")
    }

}
