use std::io::stdin;

mod commands;

mod prelude {
    pub use crate::commands::*;
}

use prelude::*;


fn main() {
    println!("Enter a command: (Type 'help' for help)");

    let mut cmd = String::new();
    stdin()
        .read_line(&mut cmd)
        .expect("Did not enter string correctly");

    run_command(cmd);
}
