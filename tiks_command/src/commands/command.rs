use std::fs::File;
use std::sync::{Mutex, RwLock};
use std::{env, fs};
use std::io::{self, BufRead, Error, ErrorKind, Read, Write};
use std::path::Path;

use async_std::task;
use lazy_static::lazy_static;


// whoami
pub fn whoami(session_context: &mut SessionContext) -> io::Result<(usize,String)>{
    let mut res = session_context.get_username();
    if session_context.user_state.root.check_permission(){
        res = "root".to_string()
    }

    Ok((STATUE_CODE,res))
}


// help 
pub fn help() -> String{
    let help = format!(
    "Usage: <command> [options] [arg]
\0\x1B[32m Commands:
    pwd     View current directory                         apt -i ..   Install package
    ls      View all files in the current directory        history     View past Commands
    cd      Change directory                               whoami  ||  apt -update version
    rm      Delete directory or file                       rn          Rename directory or file  
    touch   Create a new file                              mkdir       Create a new directory
    cat     View file only read                            mv          Move file's path
    python  Run code in python                             tar -zxvf:  Compression  
    html    Open html file                                 tar -xvf:   Decompression
    pd      Check your password                            sudo        Root
    version Get your tiks version                          exit    Exit this process\0\x1B[0m\n"
    );

    help
}


// pwd
pub fn pwd() -> io::Result<(usize,String)>{
    let path = env::current_dir().unwrap().as_path().display().to_string();
    Ok((STATUE_CODE,path))
}


// ls
pub fn ls() -> io::Result<(usize,String)> { 
    let dir_path = Path::new("./");
    let mut result = String::new();

    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                result.push_str(&format!("{}    ", entry.file_name().into_string().unwrap()));
            } else {
                result.push_str(&format!("\x1B[32m{}    \x1B[0m", entry.path().display()));
            }
        }
        Ok((STATUE_CODE,result))
    } else {
        Err(Error::new(ErrorKind::NotFound, "Path is not a directory"))
    }
}


// ll
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn ll(context: &SessionContext) -> io::Result<(usize,String)>{
    let dir_path = Path::new("./");
    let mut result = String::new();
    let dirs = fs::read_dir(dir_path)?;
    for dir in dirs{
        let dir = dir?;
        let matadata = dir.metadata()?;
        let file_type = dir.file_type()?;

        // file type
        let file_type_str = if file_type.is_dir(){
            "d"
        }else if file_type.is_file(){
            "-"
        }else if file_type.is_symlink(){
            "l"
        }else{
            "?"
        };

        // file name
        let file_name = if file_type.is_dir(){
            format!("\x1B[32m{}    \x1B[0m", dir.path().display())
        }else if file_type.is_file(){
            dir.file_name().into_string().unwrap()
        }else{
            format!("\x1B[32m{}    \x1B[0m", dir.path().display())
        };

        let mut output_or = String::new();
        let mut output_pr=String::new();
        
        // permission
        #[cfg(not(windows))]{
        use std::os::unix::fs::MetadataExt;
        let (output_o, output_p) = {
            let uid = matadata.uid();
            let gid = matadata.gid();
            let output_o = match uid {
                1000 => context.get_username(),
                0 => "root".to_string(),
                _ => "-".to_string(),
            };
            let output_p = match gid {
                1000 => context.get_username(),
                0 => "root".to_string(),
                _ => "-".to_string(),
            };
            (output_o,output_p)
        };
        output_or = output_o;
        output_pr = output_p;
        }

        let size = matadata.len();
        

        // created time
        let path = dir.path();
        let s = path.as_os_str().to_str().unwrap();
        let time = file_create_time(s);

        // output
        result.push_str(&format!(
            "{} {}  {:>8}   {:>6} {}  {}\n",
            file_type_str,
            output_pr,
            output_or,
            size,
            time,
            file_name
        ));
    }
    Ok((STATUE_CODE,result))
}


// history
lazy_static!{
    pub static ref HISTROY: Mutex<Vec<String>> = Mutex::new(Vec::new());
}


pub fn history_push(command: String){
    let mut history = HISTROY.lock().unwrap();
    history.push(command); 
}


pub fn history() -> Result<(usize,String),Error>{
    let mut output = format!("");
    let s = HISTROY.lock().unwrap();
    for (i,c) in s.iter().enumerate(){
        output.push_str(&format!("{}:{:5}",i,c));
        output.push_str("\n");
    }
    Ok((STATUE_CODE,output))
}


// cd
pub fn cd(path: &str) -> Result<(usize,String),Error>{
    let new_path = Path::new(path);
    env::set_current_dir(new_path)?;

    let res = format!("Successfully changed directory to {}.",path);
    Ok((STATUE_CODE,res))
}


// new dir
lazy_static!{
    pub static ref DIR: RwLock<&'static str> = RwLock::new("");
}


pub fn turn_dir(command: String, dir: String) -> Result<(usize,String),Error>{
    let mut dir_lock = DIR.write().unwrap();
    *dir_lock = Box::leak(dir.into_boxed_str());

    match command.as_str() {
        "mkdir" => {
            mkdir(&dir_lock)
        },
        "rm" => {
            rm(&dir_lock)
        },
        "cd" =>{
            cd(&dir_lock)
        }
        _ => {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported command",
            ))
        }
    }
}


// new file 
lazy_static! {
    pub static ref FILE: RwLock<&'static str> = RwLock::new("");
}


pub fn turn_file(command: String,file: String) -> Result<(usize,String), Error> {
    let mut file_lock = FILE.write().unwrap();
    *file_lock = Box::leak(file.into_boxed_str());

    match command.as_str() {
        "touch" => {
            touch(&file_lock)
        },
        "cat" => {
            cat(&file_lock)
        },
        "rm" => {
            rm(&file_lock)
        }
        _ =>{
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported command",
            ))
        }
    }
}


//touch
pub fn touch(file: &str) -> Result<(usize,String),std::io::Error>{
    if file.is_empty(){
        return Ok(empty_file());
    }
    let _ = fs::File::create(Path::new(file))?;

    let res = format!("Successfully created {}",file);
    Ok((STATUE_CODE,res))
}


// mkdir
pub fn mkdir(dir: &str) -> Result<(usize,String),std::io::Error>{
    if dir.is_empty(){
        return Ok(empty_dir());
    }
    let mut builder = fs::DirBuilder::new();
    let _ = builder.recursive(true).create(Path::new(dir));

    let res = format!("Successfully created {}",dir);
    Ok((STATUE_CODE,res))
}


// rm
pub fn rm(file: &str) -> Result<(usize,String),std::io::Error>{
    if file.is_empty(){
        return Ok(empty_file());
    }
    let filepath = Path::new(file);
    match filepath.is_dir() {
        true => {
            let _ = fs::remove_dir(filepath);
        },
        false => {
            let _ = fs::remove_file(filepath);
        }
    }

    let res = String::new().trim().to_owned();
    Ok((STATUE_CODE,res))
}


// rn mv
pub fn rename(source:&str,now:&str) -> std::io::Result<(usize,String)> {
    if source.is_empty(){
        return Ok(empty_file());
    }
    let _ = fs::rename(source, now);
    let res = String::new().trim().to_owned();
    Ok((STATUE_CODE,res))
}


// cat
pub fn cat(file: &str) -> Result<(usize,String),Error>{
    if file.is_empty(){
        return Ok(empty_file());
    }
    let mut buffer = String::new();
    let f = fs::File::open(Path::new(file)); 
    let _ = f.unwrap().read_to_string(&mut buffer);

    Ok((STATUE_CODE,buffer))
}


use crate::commands::apt::{download_package, find_package};
use crate::priority::get_priority;
use crate::set::set::file_create_time;
use crate::run::run;
use crate::start::state_code::{empty_dir, empty_file, env, missing_pattern, STATUE_CODE};
use super::apt::{update, update_last};
use crate::root::SessionContext;


// apt -install  xxx
pub fn apt(name: &str) -> io::Result<(usize,String)>{
    if name.is_empty(){
        return Ok(missing_pattern());
    }
    match find_package(name) {
        Some(package) => {
            if let Err(err) = download_package(&package) {
                eprintln!("Error: {}", err);
            }
        },
        None => {
            eprintln!("Package {} not found.", name);
        }
    }

    let res = format!("Successfully download Package {}",name);
    Ok((STATUE_CODE,res))
}


// apt -update xxx
pub fn update_new(version: &str) -> io::Result<(usize,String)>{
    
    match update(&version) {
        Ok(_) => {
            let res = format!("Successfully Update version {}",version);
            Ok((STATUE_CODE,res))
        }
        Err(_) => {
            let err = format!("The current version is the latest one");
            return Ok((STATUE_CODE,err));
        },
    }

}

// update lastest
#[allow(unused_must_use)]
pub fn update_lastest() -> Result<(usize,String),std::io::Error>{
    task::block_on(async {
        update_last().await;
    });
    
    return Ok((STATUE_CODE,"Successfully Update lastest version!".to_string()));
}

use tar::Archive;
use flate2::read::GzDecoder;
use flate2::Compression;
use flate2::write::GzEncoder;
use super::arg::{execute_command, execute_other_command, split, stdout_other, Commands};


pub fn zxvf(file: &str, to: &str) -> Result<(usize,String),std::io::Error>{
    if file.is_empty() || to.is_empty(){
        return Ok(missing_pattern());
    }
    let tar_gz = File::create(to)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_file(file, &mut File::open(file)?)?;
    Ok((STATUE_CODE,"Successfully Compression".to_string()))
}


pub fn xvf(to: &str) -> Result<(usize,String),std::io::Error>{
    if to.is_empty(){
        return Ok(missing_pattern());
    }
    let tar_gz = File::open(to)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok((STATUE_CODE,"Successfully Decompression".to_string()))
}


// 重定向输出   > 
#[allow(unused_variables)]
#[allow(unused_assignments)]
pub fn stdout_file(commands: Commands,session_context: &mut SessionContext) -> Result<(usize,String), std::io::Error>{
    let mut command = commands.command.clone();
    let mut option = String::new();
    let mut arg = commands.arg.clone();
    let mut file = String::new();
    if command.is_empty(){
        let (f,new_command) = stdout_other(&arg);
        file = f;
        let (command_v,option_v,arg_v) = split(new_command);
        (command,option,arg) = (command_v,option_v,arg_v);
    }else{
        file = arg[arg.len()-1].clone();
    }
    let result = execute_command(&command, &option, &arg, session_context)?.1;
    let mut file = File::create(file)?;
    file.write(result.as_bytes())?;
    Ok((STATUE_CODE,"write over!".to_string()))
}


// cp
#[allow(unused_assignments)]
pub fn cp(source:&str, to: &str) -> io::Result<(usize,String)>{
    if source.is_empty() || to.is_empty(){
        return Ok(missing_pattern());
    }

    let file = fs::read(source)?;

    let result = fs::write(to, file);
    let mut output = String::new();
    match result.is_ok(){
        true => {
            output = format!("Successfully to copy {}",to);
        },
        false =>{
            output = format!("Error: copy {} to {} failed",source,to);
        }
    }

    Ok((STATUE_CODE,output))
}


// sudo
// web in toggle root have bug (Can't show input password in the Tzcode terimal)
#[allow(unused_assignments)]
pub fn sudo(session_context: &mut SessionContext)->io::Result<(usize,String)>{
    loop{
        let mut output = String::new();
        let user = session_context.get_username();
        println!("[sudo] password for {}:",user);
        let pd = rpassword::read_password().unwrap();
        let res = session_context.toggle_root(pd);
        if res.is_ok() {
            output = format!("Sucessfully to change root");
            return Ok((STATUE_CODE,output));
        } else {
            println!("Sorry, try again");
            continue;
        }
    }
}


// get time
pub fn get_time() -> (usize,String){
    let now = chrono::Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    (STATUE_CODE,time)
}

// grep
pub fn grep(pattern:&str,arg: &str) -> io::Result<(usize,String)>{
    if arg.is_empty(){
        return Ok(missing_pattern());
    }

    let mut output = String::new();

    if let Ok(file) = File::open(arg){
        let reader = io::BufReader::new(file);

        for line in reader.lines(){
            let line = line?;
            if line.contains(pattern){
                let replaced_string = line.replace(pattern, &&format!("\x1b[31m{}\x1b[0m", pattern));
                output.push_str(&replaced_string);
                output.push_str("\n");
            }
        }
    }else {
        let string_w = arg.split_whitespace();
        for i in string_w{
            if i.contains(pattern){
                let replaced_string = i.replace(pattern, &&format!("\x1b[31m{}\x1b[0m", pattern));
                output.push_str(&replaced_string);
                output.push_str("\n");
            }
        }
    }


    if output.is_empty(){
        let _ = output.trim();
    }

    Ok((STATUE_CODE,output))
}


// | pipe
#[allow(unused_assignments)]
pub fn pipe(command:Vec<String>) -> io::Result<(usize,String)>{
    let spilt_vec = command.split(|pipe| pipe.as_str()=="|");

    let mut output = String::new();
    let mut last_result = Vec::new();

    for i in spilt_vec{
        let i = i.to_vec();
        let commands = turn_command(i);
        let (command,option,mut arg) = split(commands);
        arg.append(&mut last_result);
        let mut result = (0,String::new());
        if arg.is_empty(){
            result = execute_other_command(&command, &option, &last_result).expect("Error: ...");
        }else{
            result = execute_other_command(&command, &option, &arg).expect("Error: ...");
        }
        last_result.push(result.1.clone());
        output = result.1;
    }

    Ok((STATUE_CODE,output))
}

// &&
pub fn and(command:Vec<String>,session_context: &mut SessionContext) -> Vec<(usize,String)>{
    let mut output:Vec<_> = Vec::new();
    let commands = command.split(|x| x=="&&");
    for c in commands{
        let v = c.to_vec();
        let r = run(v, session_context);
        output.push(r)
    }
    output
}

// &
pub fn priority_run(command:Vec<String>,session_context: &mut SessionContext) -> Vec<(usize,String)>{
    let mut output:Vec<_> = Vec::new();
    let commands = command.split(|x| x=="&");
    let mut save_command = Vec::new();
    for c in commands{
        let v = c.to_vec();
        save_command.push(v);
    }

    save_command.sort_by_key(|c| -(get_priority(&c[0]).as_number() as i32));

    for c in save_command{
        let r = run(c, session_context);
        output.push(r)
    }
    output
}

fn get_env(id: String) -> String{
    let path = id.replace("$", "");
    if let Some(val) = env::var(&path).ok(){
        val
    }else{
        panic!("Error: {} is not exist",path)
    }
}

use std::path::PathBuf;
fn set_env_command(key: &str, path: &str) -> io::Result<()>{
    let mut path_set = Vec::new();
    let s = PathBuf::from(format!("{}",path));

    path_set.push(s);

    let new_path = env::join_paths(path_set).expect("Failed join paths");

    env::set_var(key, new_path);
    Ok(())
}

pub fn set(key: &str, path: &str) -> io::Result<(usize,String)>{
    match set_env_command(key, path){
        Ok(())=>{
            return Ok(env());
        },
        Err(_) =>{
            let error_str = format!("Can't set env {}",key);
            return Ok((108,error_str));
        }
    }
}

pub fn echo_print<T: std::fmt::Display + From<String>>(output: T) -> (usize,T){
    let var = format!("{}", output);
    if var.contains("$") {
        let val = get_env(var);
        return (STATUE_CODE, val.into());
    }else{
        (STATUE_CODE, output)
    }
}

// turn vec<_> to Commands
pub fn turn_command(v: Vec<String>) -> Commands{
    let ouput = Commands::new(v);
    ouput
}


// C test
#[link(name="hello")]
extern "C"{
        fn hello();
}

pub fn test_c(){
    unsafe{
        hello()
    }
}