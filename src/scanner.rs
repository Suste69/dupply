use crate::models::FileInfo;
use std::fs;
use std::path::Path;

pub fn scan_directory(path: &Path) -> anyhow::Result<Vec<FileInfo>> {
    let mut list: Vec<FileInfo> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        //println!("{}",&entry.display()); //for test
        if entry.file_type()?.is_file() {
            let size = entry.metadata()?.len();
            list.push(FileInfo {
                path: entry.path(),
                size,
            });
        } else if entry.file_type()?.is_dir() {
            list.extend(scan_directory(&entry.path())?);
        }
    }

    Ok(list)
}
