use pico_args::Arguments;
use std::process::Command;
use tempfile::TempDir;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();

    let source: String = args
        .opt_value_from_str("--source")?
        .unwrap_or_else(|| "https://github.com/lazypwny751/resourcetale".to_string());

    let destdir: String = args
        .opt_value_from_str("--destdir")?
        .unwrap_or_else(|| "bin".to_string());

    println!("Build resources/assets to binary file(s) dynamically for rusttale from resourcetale.");
    println!("Source: {}", source);
    println!("DestDir: {}", destdir);
	println!("Current Directory: {:?}", std::env::current_dir()?);

    // Check git availability
    println!("Checking for git...");
    let git_check = Command::new("git").arg("--version").output();
    match git_check {
        Ok(output) => {
            if output.status.success() {
                println!("Git found: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                return Err("Git command failed.".into());
            }
        }
        Err(_) => {
            return Err("Git not found in PATH.".into());
        }
    }

    // Create a temporary directory
    let temp_dir = TempDir::new()?;
    println!("Created temporary directory: {:?}", temp_dir.path());

    // Clone the repository into the temporary directory
    println!("Cloning {} into temporary directory...", source);
    let clone_status = Command::new("git")
        .arg("clone")
        .arg("--") // Ensure source is treated as an argument, not a flag
        .arg(&source)
        .arg(temp_dir.path())
        .status()?;

    if !clone_status.success() {
        return Err("Failed to clone repository.".into());
    }

    // Define exclude list
    let exclude_list = vec![".git"];

    // List contents of the temporary directory (excluding items in exclude_list)
    println!("Listing contents of temporary directory (excluding {:?}):", exclude_list);
    for entry in WalkDir::new(temp_dir.path()) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                
                // Get path relative to temp_dir to avoid matching system paths
                let relative_path = path.strip_prefix(temp_dir.path()).unwrap_or(path);

                // Check if any component of the path is in the exclude list
                let is_excluded = relative_path.components().any(|c| {
                    c.as_os_str()
                        .to_str()
                        .map(|s| exclude_list.contains(&s))
                        .unwrap_or(false)
                });

                if is_excluded {
                    continue;
                }
                println!("{}", path.display());
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }

    Ok(())
}
