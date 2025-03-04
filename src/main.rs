use std::io;

fn echo(cmd: &str) {
    println!("{cmd}");
}

fn main() {
    println!("Hello, welcome to Rust cmd.");

    let mut cmd = String::new();

    io::stdin()
        .read_line(&mut cmd)
        .expect("Command did not match any known commands.");

    echo(&cmd);
}
