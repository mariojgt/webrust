use std::fs;
use std::path::Path;
use std::io;

pub struct Storage;

impl Storage {
    /// Store a file in the storage/app/public directory
    pub fn put(path: &str, contents: &[u8]) -> io::Result<()> {
        let target_path = Path::new("storage/app").join(path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target_path, contents)
    }

    /// Get a file from storage
    pub fn get(path: &str) -> io::Result<Vec<u8>> {
        let target_path = Path::new("storage/app").join(path);
        fs::read(target_path)
    }

    /// Check if file exists
    pub fn exists(path: &str) -> bool {
        Path::new("storage/app").join(path).exists()
    }
    
    /// Delete a file
    pub fn delete(path: &str) -> io::Result<()> {
        let target_path = Path::new("storage/app").join(path);
        fs::remove_file(target_path)
    }
}
