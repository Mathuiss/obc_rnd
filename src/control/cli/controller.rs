use std::io::stdin;

pub fn read_input() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Failed to read from stdin");

    buf = buf.replace("\n", "");
    buf = buf.replace("\r", "");

    return buf;
}
