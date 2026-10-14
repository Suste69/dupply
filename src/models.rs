use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug)]
pub struct DuplicateGroup {
    pub hash: [u8; 32],
    pub size: u64,
    pub files: Vec<PathBuf>,
}
