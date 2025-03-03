use std::io;

fn main() {
    println!("Hello, welcome to Rust cmd.");

    let mut cmd = String::new();

    io::stdin()
        .read_line(&mut cmd)
        .expect("Command did not match any known commands.");

    let input: String = match cmd.trim().parse() {
        Ok(s) => s,
    };
}
