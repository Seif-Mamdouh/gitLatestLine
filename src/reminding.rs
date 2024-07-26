use std::process::Command;

pub fn show_latest_changes() {
    let output = Command::new("git")
        .args(&["log", "-1", "--name-only", "--pretty=format:%h %s"])
        .output()
        .expect("Failed to execute git command");

    if !output.stdout.is_empty() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut lines = output_str.lines();
        
        if let Some(commit_line) = lines.next() {
            println!("Latest commit: {}", commit_line);
            
            for file in lines {
                let file_diff = Command::new("git")
                    .args(&["diff", "HEAD~1", "HEAD", "--", file])
                    .output()
                    .expect("Failed to execute git diff command");

                let diff_str = String::from_utf8_lossy(&file_diff.stdout);
                let changed_lines: Vec<&str> = diff_str.lines()
                    .filter(|line| line.starts_with('+') || line.starts_with('-'))
                    .collect();

                if !changed_lines.is_empty() {
                    println!("File: {}", file);
                    println!("Changed lines:");
                    for line in changed_lines {
                        println!("  {}", line);
                    }
                }
            }
        }
    } else {
        println!("No changes found");
    }
}
