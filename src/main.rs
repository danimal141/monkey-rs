use std::io;

use monkey::repl;
use whoami::username;

fn main() {
    println!(
        "Hello {:?}! This is the Monkey programming language!",
        username()
    );
    println!("Feel free to type in commands");

    repl::start(io::stdin(), io::stdout());
}
