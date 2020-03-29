use std::io;

use monkey::repl;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    repl::start(io::stdin(), io::stdout());
}
