use std::process::Command;

fn main() {
    std::env::set_var("PATH", "/bin");

    let child = Command::new("ls")
        .arg("-l")
        .spawn()
        .expect("Failed to spawn child process");

    let result = child.wait_with_output().expect("Failed to wait on child");

    println!("status: {}", result.status);
    println!("Child stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("Child stderr: {}", String::from_utf8_lossy(&result.stderr));
}
