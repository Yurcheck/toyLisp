use std::{env, process};

use toy_lisp::{run_file, run_prompt, LANG_NAME};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        if let Err(err) = run_prompt() {
            println!("[{}] {}", LANG_NAME, err);
            process::exit(1);
        }
    }
    else if args.len() == 2 {
        let file_path = &args[1];

        if let Err(err) = run_file(file_path) {
            println!("[{}] {}", LANG_NAME, err);
            process::exit(1);
        }
    }
    else {
        println!("[{}] Too many arguments given", LANG_NAME);
        process::exit(1);
    }
}
