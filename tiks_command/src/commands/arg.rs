use crate::set::set::get_similar;
use crate::root::{decryption, SessionContext};
use crate::set::version;
use crate::start::state_code::{missing_pattern, not_found};

use super::code::*;
use super::command::*;

#[derive(Clone,Debug)]
pub struct Commands{
    pub command: String,
    pub option: String,
    pub arg: Vec<String>
}


#[allow(unused_assignments)]
impl Commands {
    pub fn new(commands: Vec<String>) -> Commands {
        let mut command = String::new();
        let mut option = String::new();
        let mut arg = Vec::new();
    
        // get > len
        let parts: Vec<&str> = commands.iter().map(|s| s.as_str()).collect();
        let redirect_index = parts.iter().position(|&x| x == ">");
        if let Some(redirect_index) = redirect_index {
            if redirect_index > 1 {
                arg = parts.iter().map(|&s| s.to_string()).collect();
                return Commands {
                    command: String::new(),
                    option: ">".to_string(),
                    arg,
                };
            }
        }
        match commands.as_slice() {
            [cmd] => {
                command = cmd.clone();
            }
            [cmd, opt_or_arg] => {
                command = cmd.clone();
                if opt_or_arg.starts_with("-") || opt_or_arg == ">" {
                    option = opt_or_arg.clone();
                } else {
                    arg.push(opt_or_arg.clone());
                }
            }
            [cmd, opt, args @ ..] => {
                command = cmd.clone();
                if opt.starts_with("-") || opt == ">" {
                    option = opt.clone();
                } else {
                    arg.push(opt.clone());
                }
                arg.extend_from_slice(args);
            },
            _ => {
                if commands.iter().any(|x| x == "|" || x == "&" || x == "&&" || x==">") {
                    arg = commands;
                }
            }
        }
    
        Commands {
            command,
            option,
            arg,
        }
    }

    pub fn from_string<T: Into<String>>(arg: T) -> Commands {
        let arg_string = arg.into();
        let commands: Vec<&str> = arg_string.split_whitespace().collect();
        let commands_string: Vec<String> = commands.iter().map(|x| x.to_string()).collect();
        Commands::new(commands_string)
    }
    
}


pub fn command_match(commands: Commands,session_context: &mut SessionContext) -> Result<(usize,String),std::io::Error>{
    let (command,option,arg) = split(commands.clone());
    match option.as_str() {
            ">" => stdout_file(commands, session_context),
            _ => execute_command(&command, &option, &arg, session_context),
    }
}

// root function
#[allow(unused_assignments)]
pub fn execute_command(command: &str, option: &str, arg: &Vec<String>, session_context: &mut SessionContext) -> Result<(usize,String), std::io::Error> {
    match command {
        "version" => match option{
            "-n"|"now" => {
                version::get_version_now()
            },
            "-l"|"-list" => {
                version::get_version_list()
            },
            _ =>{
                Ok((0,"help:\n  -n|now: Get version now\n  -l|list: Get all version".to_string()))
            }
        },
        "sudo" => match arg.is_empty(){
            true => {
                let output = sudo(session_context);
                output
            },
            false => {
                let output = sudo(session_context).ok();
                assert_eq!(output.unwrap().0,0);

                let command = turn_command(arg.to_vec());
                let res = command_match(command, session_context);
                session_context.user_state.exit_root();
                res
            }
        },
        "exit" => {
            match option{
                "-all" => {
                    std::process::exit(0);
                },
                _=>{
                    if session_context.user_state.root.check_permission() {
                        session_context.user_state.exit_root();
                    } else {
                        std::process::exit(0);
                    }
                }
            }
            Ok((0,"Exit".to_string()))
        },
        // error here
        "apt"=>match option{ // arg
            "-i"|"-install"=>match arg.is_empty(){
                true=>Ok(missing_pattern()),
                false=>apt(&arg[0])
            }
            "-u"|"-update"=>match arg.is_empty(){
                true=>update_lastest(),
                false=>update_new(&arg[0])
            }
            _=>Ok((0,"help:\n  -i|-install: install package \n  -u|-update: update this version".to_string()))
        },
        "export" => set(&arg[0], &arg[1]),
        "whoami" => whoami(session_context),
        "pd" => match option{  // match arg empty
            "-f"|"-fix" => match arg.is_empty(){
                true=>Ok(missing_pattern()),
                false=>session_context.user.revise_password(&arg[0])
            }
            "-c"|"-check"=>Ok({
                let pd = session_context.user.password.clone();
                let password = decryption(pd);
                (0,password)
            }),
            _=>Ok((0,"help:\n  -f|-fix: fix your password \n  -c|-check: show your password".to_string())),
        },
        "ll" => {
            let va = ll(&session_context).unwrap();
            Ok(va)
        },
        _ => execute_other_command(command, option, arg),
    }
}


// normal function
pub fn execute_other_command(command: &str, option: &str, arg: &[String]) -> Result<(usize,String), std::io::Error> {
    match command {
        "help" => Ok((0,help())),
        "pwd" => pwd(),
        "time" => Ok(get_time()),
        "history" => history(),
        // test C define here
        "hello_c" => {
            test_c();
            Ok((0,"test C ok...!".to_string()))
        },
        "ls" | "l" => ls(),
        "grep" => match arg.is_empty(){
            true=>Ok(missing_pattern()),
            false=>grep(&arg[0], &arg[1])
        },
        "echo"|"print" => Ok(echo_print(arg[0].clone())),
        "cd" | "rm" | "mkdir" | "touch" | "python" | "html" | "web" | "cat" => match arg.is_empty(){
            true=>Ok(missing_pattern()),
            false=>turn_file_or_dir(command, &arg[0])
        }
        "tar" => match option {
            "-zxvf" => match arg.is_empty(){
                true=>Ok(missing_pattern()),
                false=>zxvf(&arg[0], &arg[1]),
            } 
            "-xvf" => match arg.is_empty(){
                true=>Ok(missing_pattern()),
                false=> xvf(&arg[0]),
            }
            _ => Ok(not_found()),
        },
        "rn"|"mv" =>match arg.is_empty(){
            true=>Ok(missing_pattern()),
            false=>  rename(&arg[0], &arg[1]),
        }
        "cp"=> match arg.is_empty(){
            true=>Ok(missing_pattern()),
            false=>cp(&arg[0], &arg[1]),
        }
        _ =>{
            let similar = get_similar(&command).join("    ");
            let output = format!("
Error: Can't found this \x1B[31m{}\x1B[0m
    Did you mean?
{}", command,similar
            );
            Ok((403,output))
        }
    }
}


fn turn_file_or_dir(command: &str, arg: &str) -> Result<(usize,String), std::io::Error> {
    if let Ok(res) = turn_file(command.to_string(), arg.to_string()) {
        Ok(res)
    } else if let Ok(res) = turn_dir(command.to_string(), arg.to_string()) {
        Ok(res)
    } else if let Ok(res) = run_code(&command.to_string(), Some(arg)) {
        Ok(res)
    } else {Ok(not_found())}
}


fn run_code(command: &String,file: Option<&str>) -> Result<(usize,String),std::io::Error>{
    match command.as_str() {
        "html" | "web" => {
            html(file)
        },
        "python" | "py" => {
            python(file)
        },
        _ => Ok({
            let apt = format!("      
Command '{}' not found, did you mean:
    apt install {}
        ",command,command);
            (408,apt)
        }) 
    }
}


pub fn split(commands: Commands) -> (String,String,Vec<String>){
    let command = commands.command.clone();
    let option = commands.option.clone();
    let arg = commands.arg.clone();
    (command,option,arg)
}

pub fn stdout_other(arg: &Vec<String>) -> (String,Commands){
    let mut s = arg.split(|s| s==">");
    let output = s.next().unwrap().to_vec();
    let file = &s.next().unwrap()[0];

    
    let command = turn_command(output);

    (file.to_string(),command)
}