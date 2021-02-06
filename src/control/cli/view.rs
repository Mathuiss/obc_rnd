use std::io::stdout;
use std::io::Write;

pub fn print_msg(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}
