// This is a Simple Terimal in RUST
//  Tiks
// Provided for your reference and learning. If there are any improvements or errors.
// You can git push to main. Welcome everyone to collaborate and improve together

// There is some error or other suggestions contact me : zzj01262022@163.com
// Cargo run

use tiks_command::env::init_env;
use tiks_command::run::init_shell;
use tiks_command::start::start_logo::start_logo;
use tiks_command::root::SESSION;

fn main() {
        start_logo();

        // set os envirment path in Tiks
        init_env();

        // init shell
        init_shell(&mut *SESSION.lock().unwrap())
}