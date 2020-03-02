use std::path::PathBuf;
use std::fs::File;
use std::io;

#[warn(unused_imports)]
pub(crate) fn cover(file_path: PathBuf) -> std::io::Result<()> {
    if file_path.is_file() {
        println!("FILE : {}", file_path.is_file());
        permissions(&file_path);
    }
    if file_path.is_dir() {
        println!("DIR : {}", file_path.is_dir())
    }
    return Ok(());
}


fn permissions(path: &PathBuf) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut perms = file.metadata()?.permissions();
    perms.set_readonly(true);
    file.set_permissions(perms)?;
    Ok(())
}