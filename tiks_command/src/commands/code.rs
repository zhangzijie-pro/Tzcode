use std::process::Command;

use crate::start::state_code::{run_code, run_code_er};

// run code use python ...
pub fn python(file: Option<&str>) -> Result<(usize,String), std::io::Error> {
    if file.is_none(){
        eprintln!("Error: please provide a valid file")
    }
    let mut binding = Command::new("python3");
    let cmd = binding
    .arg(file.unwrap());

    let s = cmd.spawn()?.wait();

    if s.is_err(){
        return Ok(run_code_er());
    }

    Ok(run_code())
}


// open web in html
pub fn html(file: Option<&str>) -> Result<(usize,String),std::io::Error>{
    match file {
        Some(html) => {
            let s = webbrowser::open(html);
            if s.is_err(){
                return Ok(run_code_er());
            }
        },
        None => {
            eprintln!("Error: Path is None. Please provide a valid file path or web.");
        }
    }
    Ok(run_code())
}

// others