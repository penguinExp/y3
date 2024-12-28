use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Reader {
    paths: Vec<String>,
    ignore_patterns: Option<GlobSet>,
    base_dir: PathBuf,
}

impl Reader {
    pub fn new(base_dir: &str) -> Self {
        Self {
            paths: Vec::new(),
            ignore_patterns: None,
            base_dir: PathBuf::from(base_dir),
        }
    }

    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    ///
    /// Extract file paths from the current directory and return the count
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the directory or a specific file
    ///
    pub fn get_files(&mut self, path: &str) -> io::Result<usize> {
        let metadata = fs::metadata(path)?;

        if metadata.is_file() {
            if self.should_ignore(path) {
                return Ok(0);
            }
            self.paths.push(path.to_string());
            return Ok(1);
        }

        if metadata.is_dir() {
            // Ignore the .git folder
            if path.ends_with(".git") {
                return Ok(0);
            }

            let mut count = 0;
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let entry_str = entry_path.to_str().unwrap_or_default();

                if entry_path.is_dir() {
                    count += self.get_files(entry_str)?;
                } else if entry_path.is_file() {
                    if !self.should_ignore(entry_str) {
                        self.paths.push(entry_str.to_string());
                        count += 1;
                    }
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

    ///
    /// Parse .gitignore file and load ignore patterns
    ///
    /// # Arguments
    ///
    /// * `gitignore_path` - Path to the .gitignore file
    ///
    pub fn load_gitignore(&mut self) -> io::Result<()> {
        let gitignore_path = self.base_dir.join(".gitignore");
        if !gitignore_path.exists() {
            return Ok(()); // No .gitignore file, nothing to do
        }

        let gitignore_content = fs::read_to_string(&gitignore_path)?;
        let mut builder = GlobSetBuilder::new();

        for line in gitignore_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            // Convert each pattern to a full path relative to the base directory
            let pattern = self.base_dir.join(trimmed);
            let glob_pattern = GlobBuilder::new(pattern.to_str().unwrap_or_default())
                .literal_separator(true)
                .build()
                .map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid glob pattern: {err}"),
                    )
                })?;
            builder.add(glob_pattern);
        }

        self.ignore_patterns = Some(builder.build().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Failed to build glob set: {err}"),
            )
        })?);

        Ok(())
    }

    ///
    /// Check if a path should be ignored based on the loaded ignore patterns
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    fn should_ignore(&self, path: &str) -> bool {
        if let Some(ref patterns) = self.ignore_patterns {
            patterns.is_match(Path::new(path))
        } else {
            false
        }
    }
}
