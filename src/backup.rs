use std::{process::Command, thread};

fn main() {
    let backups: Vec<&str> = vec![
        "layers",
        "photos",
        "admins",
        "points",
        "users",
        "projects",
        "companies",
    ];

    for backup in backups {
        thread::sleep(std::time::Duration::new(5, 0));
        execute_backup(backup);
    }
}

fn execute_backup(backup_name: &str) {
    println!("Backuping: {}", backup_name);
    let output: std::process::Output =
        Command::new(format!("./target/release/backup-{}", backup_name))
            .output()
            .expect(format!("failed to backup {}\n.", backup_name).as_str());

    println!("status: {}", output.status);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    assert!(output.status.success());
}
