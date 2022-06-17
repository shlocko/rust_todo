use std::io::stdin;

mod commands;
mod items;

mod prelude {
    pub use crate::commands::*;
    pub use crate::items::*;
}

use prelude::*;


fn main() {
    let mut items = Items::new();
    println!("Enter a command: (Type 'help' for help)");

    // Read a command from the user
    loop {
        let mut cmd = String::new();
        stdin()
            .read_line(&mut cmd)
            .expect("Did not enter string correctly");

        if let Err(message) = run_command(&mut items, cmd.trim().to_string()) {
            println!("{}", message);
        }
    }

}
