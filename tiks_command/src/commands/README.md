## 添加方法到Rust

- 在command.rs中添加方法 (可在test.rs中检测方法)
- arg.rs的execute_other_command方法中添加
- 如果要设置root权限才可使用可在root.rs中allowed_commands中添加


例如:
``` rust
1. command.rs
    pub fn vim(filename: &str) -> (usize,String){
        todo()!
    }

2. arg.rs
    pub fn execute_other_command(command: &str, option: &str, arg: &[String]) -> Result<(usize,String), std::io::Error> {
        match command {
            "vim" => vim(&arg[0])
        }
    }

3. 修改root
    impl SessionContext{
        pub fn new() -> SessionContext{
            let root = Root{
                allowed_commands: vec![
                "vim".to_string()], // add command in root
            };
        }
    }
    
```