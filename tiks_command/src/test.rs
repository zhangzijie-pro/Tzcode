#[cfg(test)]
mod tests {
    use crate::{commands::command::echo_print, env::init_env};

    #[test]
    #[ignore]
    fn detection_func() {
        let input = "$YOUR_env";
        let (ouput_n,output_s) = echo_print(input.to_string());
        assert_eq!(0,ouput_n);      // passed
        assert_eq!("your_env_value",output_s)  // value
    }

    #[test]
    fn env_set() {
        let s = init_env();         // set env in your os test
        assert_eq!((),s);
    }

    #[test]
    fn your_function(){

    }
}
