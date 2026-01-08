use std::path::{Path, PathBuf};

use crate::position::Position;

#[derive(Debug)]
pub struct Tokenizer {
    path: Option<PathBuf>,
    content: String,
    position: Position,
}

impl Tokenizer {
    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self {
            path: Some(path.to_path_buf()),
            content,
            position: Position::default(),
        })
    }

    fn from_str(content_str: &str) -> Self {
        Self {
            path: None,
            content: content_str.to_string(),
            position: Position::default(),
        }
    }

    fn is_eof(&self) -> bool {
        self.position.at >= self.content.chars().count()
    }

    fn current_char(&self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        self.content.chars().nth(self.position.at)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use tempfile::tempdir;

    use crate::tokenizer::Tokenizer;

    #[test]
    fn from_file() {
        let tokenizer = Tokenizer::from_file(Path::new(""));
        assert!(tokenizer.is_err());

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("temporary.txt");
        File::create(&file_path).unwrap();
        let tokenizer = Tokenizer::from_file(file_path.as_path());
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn from_string() {
        let tokenizer = Tokenizer::from_str("content");
        assert_eq!(tokenizer.path, None)
    }

    #[test]
    fn is_eof() {
        let tokenizer = Tokenizer::from_str("content");
        assert!(!tokenizer.is_eof());
        assert_eq!(tokenizer.current_char(), Some('c'));
    }
}
