use std::fs;
use std::path;

use anyhow::Context;

use crate::error::Error;
use crate::error::Result;
use crate::key;

#[derive(Debug)]
pub struct FileInfo {
    pub filepath: String, // full file path
    pub extension: String,
    pub key: String,
}

pub struct FileInfoConstructor<'a> {
    key_extractor: &'a dyn key::Extractor,
}

impl FileInfoConstructor<'_> {
    pub fn new(key_extractor: &dyn key::Extractor) -> FileInfoConstructor {
        return FileInfoConstructor { key_extractor };
    }

    pub fn from_dir(&self, dir: &str, extensions: &Vec<String>) -> Result<Vec<FileInfo>> {
        let dir_entries = fs::read_dir(dir).with_context(|| format!("read dir {}", dir))?;

        return self.from_paths(dir_entries, extensions);
    }

    fn from_paths(
        &self,
        dir_entries: fs::ReadDir,
        extensions: &Vec<String>,
    ) -> Result<Vec<FileInfo>> {
        let mut file_info_list = vec![];
        for dir_entry in dir_entries {
            let file_path = dir_entry
                .with_context(|| format!("get filepath from dir entry"))?
                .path();

            match self.gen_file_info(&file_path, extensions) {
                Ok(file_info) => file_info_list.push(file_info),
                Err(err) => debug!("failed to gen file({:?}) info: {:?}", file_path, err),
            }
        }

        Ok(file_info_list)
    }

    fn gen_file_info(
        &self,
        file_path: &path::PathBuf,
        extensions: &Vec<String>,
    ) -> Result<FileInfo> {
        let file_ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or(Error::msg("without extension"))?;

        if !extensions.contains(&file_ext.to_string()) {
            return Result::Err(Error::msg(format!(
                "ignore file with extension {:?}",
                file_ext,
            )));
        }

        let full_file_path = file_path
            .to_str()
            .ok_or(Error::msg("failed to get file full path"))?;
        let filename = file_path
            .file_name()
            .and_then(|e| e.to_str())
            .ok_or(Error::msg("file to get file name"))?;
        let key = self
            .key_extractor
            .extract(filename)
            .ok_or(Error::msg(format!(
                "failed to extract key from file name: {:?}",
                filename
            )))?;

        return Ok(FileInfo {
            filepath: full_file_path.to_string(),
            extension: file_ext.to_string(),
            key,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{path, vec};

    use super::FileInfoConstructor;
    use crate::key::Extractor;

    struct TestKeyExtractor {
        key: Option<String>,
    }
    impl Extractor for TestKeyExtractor {
        fn extract(&self, _: &str) -> Option<String> {
            return self.key.clone();
        }
    }

    #[test]
    fn test_gen_file_info_without_extension() {
        let key_extractor = TestKeyExtractor { key: None };

        let file_info_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "test";
        let file_info = file_info_constructor.gen_file_info(
            &path::PathBuf::from(filepath.to_string()),
            &vec!["txt".to_string()],
        );
        assert_eq!(false, file_info.is_ok());
    }

    #[test]
    fn test_gen_file_info_extension_miss() {
        let key_extractor = TestKeyExtractor { key: None };

        let file_info_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "test.txt";
        let file_info = file_info_constructor.gen_file_info(
            &path::PathBuf::from(filepath.to_string()),
            &vec!["png".to_string()],
        );
        assert_eq!(false, file_info.is_ok());
    }

    #[test]
    fn test_gen_file_info_extractor_none() {
        let key_extractor = TestKeyExtractor { key: None };

        let file_info_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "test.txt";
        let file_info = file_info_constructor.gen_file_info(
            &path::PathBuf::from(filepath.to_string()),
            &vec!["txt".to_string()],
        );
        assert_eq!(false, file_info.is_ok());
    }

    #[test]
    fn test_file_info_constructor() {
        let key = "test_key";
        let key_extractor = TestKeyExtractor {
            key: Some(key.to_string()),
        };

        let file_info_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "./dir/test.txt";
        let file_info = file_info_constructor.gen_file_info(
            &path::PathBuf::from(filepath.to_string()),
            &vec!["txt".to_string()],
        );
        assert_eq!(true, file_info.is_ok());

        let file_info = file_info.unwrap();
        println!("return file info: {:?}", file_info);
        assert_eq!(filepath.to_string(), file_info.filepath);
        assert_eq!("txt", file_info.extension);
        assert_eq!(key, file_info.key);
    }
}
