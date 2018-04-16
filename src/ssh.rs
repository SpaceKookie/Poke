//! An ssh runner utility

use std::process::Command;

/// Generate a new key and store it in $USER/.ssh/poke
pub fn generate_key(path: &str, name: &str) {
    let foo = Command::new("ssh-keygen")
        .arg("-t")
        .arg("ed25519")
        .arg("-f")
        .arg(&format!("{}/{}", path, name))
        .arg("-N")
        .arg("''")
        .output()
        .expect("Failed to execute `ssh-keygen`");

    print!("{}", String::from_utf8(foo.stdout).unwrap());
}

/// Register a new key with a remote server
pub fn send_key(file_path: &str, server: &str) {
    println!("Adding public key to server...");
    let foo = Command::new("scp")
        .arg(file_path)
        .arg(&format!("{}:~/.ssh", server))
        .output()
        .expect("Failed to copy key to server!");

    println!("{}", String::from_utf8(foo.stdout).unwrap());
    println!("{}", String::from_utf8(foo.stderr).unwrap());
}
