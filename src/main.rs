// Unix Shell in Rust
// NOTE: Too simple... please use zsh instead :sob:

use std::{
    env::{current_dir, set_current_dir},
    error::Error,
    io::{self, Write},
    process::Command,
};

const INIT: &str = "$> ";

fn run_cmd(cmd: &str, args: &[&str]) {
    // use std::process::Command
    let status = Command::new(cmd).args(args).status();

    match status {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn cd(args: &[&str]) {
    // implementing our own cd
    if args.len() != 1 {
        eprintln!("usage: cd <dir>");
        return;
    }
    // using std::env::set_current_dir
    match set_current_dir(args[0]) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn shell() -> Result<(), Box<dyn Error>> {
    loop {
        // print currentdir and init
        let cur_dir = current_dir()?;
        print!("{}{}", cur_dir.display(), INIT);
        io::stdout().flush()?;

        // accept input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        //check for empty
        if input.is_empty() {
            continue;
        }

        // parse args
        let parts: Vec<&str> = input.split_whitespace().collect();
        let args = &parts[1..];
        let cmd = parts[0];

        // execute commands
        match cmd {
            "cd" => cd(args),
            "exit" => return Ok(()),
            other => run_cmd(other, args),
        }
    }
}

fn main() {
    if let Err(e) = shell() {
        eprintln!("Error: {e}");
    }
}
