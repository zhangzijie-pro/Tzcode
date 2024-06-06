#[cfg(windows)]
pub fn start_logo() {
    println!("Welcome to Tiks (Simple Linux terminal)");
    println!(
        "\x1B[31m{}\x1B[0m",
        r#"
  _______ _ _      
 |__   __(_) |     
    | |   _| |_ ___
    | |  | | __/ __|
    | |  | | |_\__ \
    |_|  |_|\__|___/
                    
"#
    );
println!("  * Contact us :  zzj01262022@163.com");
println!("  * Github :  https://github.com/zhangzijie-pro/Tiks/");
println!("This message is shown once a day. Commands are saved only when the application is running")
}

#[cfg(not(windows))]
pub fn start_logo() {
    use termion::{color, style};
    print!("{}[2J", 27 as char);
    // terimal width
    let terminal_width = match term_size::dimensions() {
        Some((w, _)) => w,
        None => 80,
    };

    let title = "Welcome to Tiks (Simple Linux terminal)";
    let padding = (terminal_width - title.len()) / 2;
    println!("{:=^width$}", "", width = terminal_width);
    println!("{:width$}", "", width = padding);

    println!(
        "{}{}{}",
        style::Bold,
        title,
        style::Reset
    );
    println!("{:width$}", "", width = padding);
    println!("{:=^width$}", "", width = terminal_width);
    println!();

    // logo
    println!("{:width$}", "", width = padding);
    println!(
        "{}{}{}",
        color::Fg(color::Red),
        "  _______ _ _",
        style::Reset
    );
    println!(
        "{}{}{}",
        color::Fg(color::Blue),
        " |__   __(_) |",
        style::Reset
    );
    println!(
        "{}{}{}",
        color::Fg(color::Green),
        "    | |   _| |_ ___",
        style::Reset
    );
    println!(
        "{}{}{}",
        color::Fg(color::Yellow),
        "    | |  | | __/ __|",
        style::Reset
    );
    println!(
        "{}{}{}",
        color::Fg(color::Cyan),
        "    | |  | | |\\__ \\",
        style::Reset
    );
    println!(
        "{}{}{}",
        color::Fg(color::Magenta),
        "    |_|  |_|\\__|___/",
        style::Reset
    );
    println!("{:width$}", "", width = padding);
    println!();

    // github email
    println!(
        "{}{}{}",
        color::Fg(color::Reset),
        " * Contact us :  zzj01262022@163.com",
        style::Reset
    );
    println!(
        "{}{}{}",
        color::Fg(color::Reset),
        " * Github :  https://github.com/zhangzijie-pro/Tiks/",
        style::Reset
    );
    println!();

    // time
    let now = chrono::Local::now();
    println!(
        "{}Current Date & Time: {}",
        style::Italic,
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    println!();

    // message
    println!(
        "{}This message is shown once a day. Commands are saved only when the application is running{}",
        color::Fg(color::LightBlack),
        style::Reset
    );
    println!("{:=^width$}", "", width = terminal_width);
    println!();
}
