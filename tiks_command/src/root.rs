// build root
pub struct UserState{
    pub root: UserRole
}

pub enum UserRole {
    User,
    Admin,
}

impl UserRole{
    pub fn check_permission(&self) -> bool {
        match self {
            UserRole::Admin => true, 
            UserRole::User => false, 
        }
    }
}


pub struct Root {
    pub allowed_commands: Vec<String>,
}

impl UserState {
    pub fn new() -> UserState{
        let root = UserRole::User;
        UserState{
            root
        }
    }

    pub fn toggle_root(&mut self){
        self.root = UserRole::Admin;
    }

    pub fn exit_root(&mut self){
        self.root=UserRole::User;
    }
}

use std::io::{self,Write};
use std::fs;
use std::process::Command;


pub struct User{
    pub username: String,
    pub password: String,
    has_set_password: bool,
}

impl User{
    pub fn new(username: String, password: String, has_set_password: bool) -> User {
        User {
            username,
            password,
            has_set_password,
        }
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
        self.has_set_password = true;
    }

    pub fn revise_password(&self, password: &str) -> Result<(usize,String), std::io::Error> {
        let new_pd = encryption(password.to_string());
        let _ = self.password == new_pd;

        let output = format!("Successfully revises the password");
        Ok((STATUE_CODE,output))
    }

    pub fn has_set_password(&self) -> bool {
        self.has_set_password
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized_user = format!("{}:{}", self.username, self.password);
        fs::write(filename, serialized_user)
    }

    pub fn load_from_file(filename: &str) -> io::Result<User> {
        let serialized_user = fs::read_to_string(filename)?;
        let mut parts = serialized_user.split(':');
        let username = parts.next().unwrap_or("").to_string();
        let password = parts.next().unwrap_or("").to_string();
        Ok(User {
            username,
            password,
            has_set_password: true,
        })
    }
}


pub struct SessionContext {
    pub root: Root,
    pub user_state: UserState,
    pub user: User
}

impl SessionContext{
    pub fn new() -> SessionContext{
        let root = Root{
            allowed_commands: vec![
            "mkdir".to_string(),"rm".to_string(),"rn".to_string(),
            "touch".to_string(),"cat".to_string(),"mv".to_string(),
            "pd".to_string()], // add command in root
        };
        let userstate = UserState::new(); //false

        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let binding = home_dir.join(".Tiks").join("tiks");  // create dir in setup.sh
        let user_file_path = binding.as_os_str().to_str().unwrap();

        let user = match User::load_from_file(&user_file_path){
            Ok(res) =>{
                 if !res.has_set_password() {
                    let mut username = String::new();

                    get_username(&mut username);
                    let password = get_password();
    
                    let user = User::new(username, password, false);
                    user.save_to_file(&user_file_path).expect("Failed to save user");
                    user
                }else{
                    res
                }
            },
            Err(_) =>{
                println!("Tiks::welcome-to-try\r\nplease add your account");
                #[cfg(target_os="linux")]
                init_setup_linux();
                #[cfg(target_os="macos")]
                init_setup_mac();
                #[cfg(target_os="windows")]
                init_setup_windows();
                let mut user = String::new();

                get_username(&mut user);
                let password = get_password();
                let pd = encryption(password);

                let user = User::new(user, pd, false);
                user.save_to_file(&user_file_path).expect("Failed to save user");
                user
            }
        };
        
        SessionContext{
            root, 
            user_state: userstate,
            user
        }
    }
    pub fn get_username(&self) -> String{
        let name = self.user.username.clone().trim().to_string();
        name
    }

    
    pub fn toggle_root(&mut self,password:String) -> io::Result<()>{
        let user_pd = self.user.password.clone();
        let pd = decryption(user_pd);
        if password == pd {
            self.user_state.toggle_root();
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Incorrect password"))
        }
    }
}

fn get_username(user: &mut String){
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(user).unwrap();
}

fn get_password() -> String{
    loop {
        println!("Enter password:");
        let pd = rpassword::read_password().unwrap();
        println!("Enter password again:");
        let pd_again = rpassword::read_password().unwrap();
        if pd == pd_again{
            return pd
        }else {
            eprint!("Error: password isn't same\n");
            continue;
        }
    }
}

use std::str;
use std::sync::Mutex;
#[allow(deprecated)]
use base64::{encode, decode};
use lazy_static::lazy_static;

use crate::start::state_code::STATUE_CODE;
// base64
// 加密
#[allow(deprecated)]
fn encryption(pd: String) -> String{
    let res = encode(pd.clone());
    res
}

// 解密
#[allow(deprecated)]
pub fn decryption(pd: String) -> String{
    let code = decode(pd.clone());
    match code{
        Ok(res) =>{
            let password = std::str::from_utf8(&res).unwrap().to_string();
            password
        },
        Err(_)=>{
            let output = format!("The password does not exist");
            output
        }
    }
}

lazy_static!{
    pub static ref SESSION: Mutex<SessionContext> = Mutex::new(SessionContext::new());
}

// for every os
#[cfg(target_os="linux")]
fn init_setup_linux(){
    Command::new("bash")
    .arg("./mac_linux/setup.sh")
    .spawn()
    .expect("Error: Can't setup");
}

#[cfg(target_os="macos")]
fn init_setup_mac() {
    Command::new("bash")
        .arg("./mac_linux/setup.sh")
        .spawn()
        .expect("Error: Can't setup on macOS");
}

#[cfg(target_os="windows")]
fn init_setup_windows() {
    Command::new("./window/setup.bat")
        .spawn()
        .expect("Error: Can't setup on Windows");
}