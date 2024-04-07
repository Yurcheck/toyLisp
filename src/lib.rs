use std::error::Error;

mod token;

pub const LANG_NAME: &'static str = "ToyLISP";

pub fn run_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("{}", file_path);

    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    todo!();
}

pub fn run(source: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}
