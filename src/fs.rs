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
