#[cfg(target_os="linux")]
fn set_env() -> (usize,String){
    use std::{env, path::PathBuf};

    use crate::start::state_code::env;

    let mut path = match env::var_os("PATH"){
        Some(val) => env::split_paths(&val).collect::<Vec<_>>(),
        None=>Vec::new(),
    };

    let home_dir = match env::var_os("HOME"){
        Some(val) => val.into_string().expect("HOME not found"),
        None=> panic!("Error: HOME not set"),
    };
    
    let tiks_bin = PathBuf::from(format!("{}/.Tiks/bin",home_dir));

    path.push(tiks_bin);

    let new_path = env::join_paths(path).expect("Failed join paths");

    env::set_var("PATH", new_path);

    env()
}

#[cfg(target_os="windows")]
fn set_env() -> (usize,String){
    use std::env;
    use std::path::PathBuf;

    use crate::start::state_code::env;

    let path = env::var_os("PATH").unwrap_or_default();
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();

    let home_dir = env::var("USERPROFILE").expect("Failed to get USERPROFILE");
    let tiks_bin = PathBuf::from(format!("{}/.Tiks/bin",home_dir));

    paths.push(tiks_bin);

    let new_path = env::join_paths(paths).expect("Failed to join paths");
    env::set_var("PATH", new_path);

    env()
}

pub fn init_env(){
    #[cfg(target_os="windows")]
    set_env();
    #[cfg(target_os="linux")]
    set_env();
}
