use std::process::Command;

fn main() {
    std::env::set_var("PATH", "/bin");

    let mut cmd = Command::new("ls");
    cmd.arg("-l");

    let output = cmd.output().expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
