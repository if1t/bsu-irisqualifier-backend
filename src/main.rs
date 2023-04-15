use std::io::Write;
use std::process::Command;

fn main() {
    let output = Command::new("irisqualifier-method")
        .output()
        .expect("Failed to execute command");

    let mut out = std::io::stdout();
    out.write_all(output.stdout.as_slice())
        .expect("Failed to output result");
}
