use std::process::Command;
use std::fs;

pub fn show_latest_changes() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                // Get the latest commit message
                let output = Command::new("git")
                    .args(&["log", "-1", "--pretty=format:%h %s", &path.to_string_lossy()])
                    .output()
                    .expect("Failed to execute git command");

                if !output.stdout.is_empty() {
                    println!("File: {:?}, Change: {}", path.file_name().unwrap(), String::from_utf8_lossy(&output.stdout));
                    
                    // Get the actual lines changed
                    let diff_output = Command::new("git")
                        .args(&["diff", &path.to_string_lossy()])
                        .output()
                        .expect("Failed to execute git diff command");

                    if !diff_output.stdout.is_empty() {
                        println!("Changes:\n{}", String::from_utf8_lossy(&diff_output.stdout));
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory");
    }
}


