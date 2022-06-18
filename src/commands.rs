use crate::prelude::*;

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
pub fn run_command(items: &mut Items, input: String) -> Result<(), String> {
    let parsed_command = parse_input(input); // Prarse the input

    match parsed_command {
        Some((cmd, args)) => { // Checks that there was input
            match cmd.as_str() { // Matches the given command against the list of possible commands
                "test" => {
                    for s in args {
                        println!("{}", s);
                    }
                },
                "add" => {
                    let mut content = String::new();
                    for s in args {
                        content.push(' ');
                        content.push_str(&s);
                    }
                    items.add_entry(content);
                },
                "list" => {
                    for i in 0..items.get_entries().len() {
                        println!("Entry {}: {}", i + 1, items.get_entries()[i].get_content());
                    }
                },
                "delete" => {
                    if args.len() == 1 {
                        match args[0].parse::<usize>() {
                            Ok(n) => {
                                match items.del_entry(n) {
                                    Ok(()) => println!("Item deleted"),
                                    Err(()) => return Err("Invalid index".to_string())
                                }
                            },
                            Err(_e) => return Err("Argument provided was not a valid number".to_string())
                        }
                    } else {
                        return Err("Usage: delete <entry number>".to_string());
                    }
                },
                _ => return Err("Invalid command.".to_string())
            }
        },
        None => return Err("Please enter a command:".to_string())
    }
    Ok(())
}
