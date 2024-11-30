use std::{fs, io};

pub struct Reader {
    paths: Vec<String>,
}

impl Reader {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    ///
    /// Extract file paths from the current directory and return the count
    ///
    /// # Argument
    ///
    /// * `path` - Path to the directory or a specific file
    ///
    pub fn get_files(&mut self, path: &str) -> io::Result<usize> {
        let metadata = fs::metadata(path)?;

        if metadata.is_file() {
            self.paths.push(path.to_string());

            return Ok(1);
        }

        if metadata.is_dir() {
            let mut count = 0;

            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let entry_str = entry_path.to_str().unwrap_or_default().to_string();

                if entry_path.is_dir() {
                    count += self.get_files(&entry_str)?;
                } else if entry_path.is_file() {
                    self.paths.push(entry_str);

                    count += 1;
                }
            }

            return Ok(count);
        }

        // If the path is neither file nor directory, return an error
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The provided path is neither a file nor a directory.",
        ))
    }
}
