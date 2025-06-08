/// Sets up the directory at ~/.repo-depot/ to clone repos into
pub fn set_up_dir() -> Result<(), Box<dyn std::error::Error>> {
    let path_name = ".repo-depot";
    match dirs::home_dir() {
        Some(home) => {
            let path = home.join(path_name);
            if !path.exists() {
                std::fs::create_dir_all(&path)?;
            }
            Ok(())
        }
        None => {
            eprintln!("No home directory found");
            std::process::exit(1);
        }
    }
}

// git clone <git_url> <path>/<repo_name>
pub fn clone_repos(repos: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for repo in repos {
        let repo_name = repo.split("/").last().unwrap();
        let path = dirs::home_dir()
            .unwrap()
            .join(".repo-depot")
            .join(repo_name);
        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(&repo)
            .arg(path)
            .output()?;

        if !output.status.success() {
            eprintln!("Failed to clone repo: {}", repo_name);
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        } else {
            println!("Cloned repo: {}", repo_name);
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }

    Ok(())
}
