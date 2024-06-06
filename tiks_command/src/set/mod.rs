pub mod set;


pub mod version{
    use std::io::{self, Read};

    use crate::start::state_code::STATUE_CODE;

    pub fn get_version_list() -> io::Result<(usize,String)>{
        let mut s = String::new();
        let mut res = std::fs::File::open("command/test/version").unwrap();
        let _ = res.read_to_string(&mut s);

        Ok((STATUE_CODE,s))
    }

    pub fn get_version_now() -> io::Result<(usize,String)>{
        let s = env!("CARGO_PKG_VERSION").parse::<String>().unwrap();
        Ok((STATUE_CODE,s))
    }
}