#[cfg(test)]
mod tests {
    use crate::get_file_language;
    use crate::read_workspace_config;
    #[test]
    fn language(){
        let filename: String = String::from("main.rs");
        let language = get_file_language(filename);
        assert_eq!(language,String::from("rust"))
    }

    #[test]
    fn read_workspace(){
        let config = vec!["C:\\Users\\lenovo\\Desktop\\rust".to_string()];
        let res = read_workspace_config().expect("Can't read").workspace;
        assert_eq!(config,res)
    }
}
