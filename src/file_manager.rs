use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct FileManager {
    path: String,
}

impl FileManager {
    pub fn new(path: &str) -> Self {
        FileManager {
            path: path.to_string(),
        }
    }

    pub fn read_file(&self) -> io::Result<Vec<u8>> {
        let mut file = File::open(&self.path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_file(&self, data: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)?;
        file.write_all(data)?;
        Ok(())
    }
}
