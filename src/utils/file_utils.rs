use directories::ProjectDirs;
use std::{fs, io, path::PathBuf};

pub fn app_path() -> io::Result<PathBuf> {
    // en Windows: C:\Users\USERNAME\AppData\Local\Thekes\data
    #[cfg(target_os = "windows")]
    let dirs = ProjectDirs::from("com", "Thekes", "");

    // en Linux: /home/USERNAME/.local/share/thekes/"
    #[cfg(target_os = "linux")]
    let dirs = ProjectDirs::from("com", "Thekes", "Thekes");

    match dirs {
        Some(dirs) => Ok(dirs.data_local_dir().to_path_buf()),
        None => Ok(std::env::current_dir()?.join("data")),
    }
}

pub fn db_path() -> io::Result<PathBuf> {
    let data_dir = app_path()?;
    //fs::create_dir_all(&data_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&data_dir)
        .map_err(|err| io::Error::new(err.kind(), format!("No se pudo crear {}: {err}", data_dir.display())))?;

    Ok(data_dir.join("thekes.db"))
}
