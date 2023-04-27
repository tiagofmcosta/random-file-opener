use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use rand::seq::SliceRandom;

fn get_all_files_recursively(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Ignore hidden files and folders
            if let Some(file_name) = path.file_name() {
                if file_name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            if path.is_dir() {
                paths.extend(get_all_files_recursively(&path)?);
            } else {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let files = get_all_files_recursively(&current_dir)?;

    let mut rng = rand::thread_rng();
    let random_file = files.choose(&mut rng);

    match random_file {
        Some(path) => {
            if let Some(file_name) = path.file_name() {
                println!("Opening '{}'...", file_name.to_string_lossy());
                let path_str = path.display();
                let result = Command::new("start").arg(path_str.to_string()).spawn();

                if let Err(error) = result {
                    if let io::ErrorKind::NotFound = error.kind() {
                        Command::new("PowerShell.exe")
                            .arg("-Command")
                            .arg(format!("& {{Start-Process \"{}\"}}", path_str))
                            .spawn()?;
                    }
                }
            }
        }
        None => println!("There are no files in the current directory"),
    }

    Ok(())
}
