use std::fs;
use std::path;

use anyhow::Context;

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
        let paths = fs::read_dir(dir).with_context(|| format!("read dir {}", dir))?;

        return self.from_paths(paths, extensions);
    }

    fn from_paths(&self, paths: fs::ReadDir, extensions: &Vec<String>) -> Result<Vec<FileInfo>> {
        let mut ret = Vec::<FileInfo>::new();
        for path in paths {
            let filepath = path
                .with_context(|| format!("get filepath from dir entry"))?
                .path();

            if let Some(fileinfo) = self.gen_fileinfo(filepath, extensions)? {
                ret.push(fileinfo);
            }
        }

        Ok(ret)
    }

    fn gen_fileinfo(
        &self,
        filepath: path::PathBuf,
        extensions: &Vec<String>,
    ) -> Result<Option<FileInfo>> {
        if let Some(filepath_extenstion) = filepath.extension() {
            for extension in extensions {
                if filepath_extenstion != extension.as_str() {
                    continue;
                }

                if let Some(full_filepath_str) = filepath.to_str() {
                    if let Some(filename_str) = filepath.file_name() {
                        let filename_str = filename_str.to_str().unwrap();
                        if let Some(key) = self.key_extractor.extract(filename_str) {
                            return Ok(Some(FileInfo {
                                filepath: full_filepath_str.to_string(),
                                extension: extension.to_string(),
                                key,
                            }));
                        } else {
                            debug!(
                                "extractor returns none, skip filepath {:?}",
                                full_filepath_str
                            );

                            return Ok(None);
                        }
                    }
                }
            }

            debug!(
                "does not match any extensions, skip file {:?}, extension: {:?}",
                filepath, filepath_extenstion
            );
        } else {
            debug!("without extension, skip file {:?}", filepath);
        }

        Ok(None)
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
    fn test_gen_fileinfo_without_extension() {
        let key_extractor = TestKeyExtractor { key: None };

        let fileinfo_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "test";
        let fileinfo = fileinfo_constructor.gen_fileinfo(
            path::PathBuf::from(filepath.to_string()),
            &vec!["txt".to_string()],
        );
        assert_eq!(true, fileinfo.is_ok());
        assert_eq!(true, fileinfo.unwrap().is_none());
    }

    #[test]
    fn test_gen_fileinfo_extension_miss() {
        let key_extractor = TestKeyExtractor { key: None };

        let fileinfo_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "test.txt";
        let fileinfo = fileinfo_constructor.gen_fileinfo(
            path::PathBuf::from(filepath.to_string()),
            &vec!["png".to_string()],
        );
        assert_eq!(true, fileinfo.is_ok());
        assert_eq!(true, fileinfo.unwrap().is_none());
    }

    #[test]
    fn test_gen_fileinfo_extractor_none() {
        let key_extractor = TestKeyExtractor { key: None };

        let fileinfo_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "test.txt";
        let fileinfo = fileinfo_constructor.gen_fileinfo(
            path::PathBuf::from(filepath.to_string()),
            &vec!["txt".to_string()],
        );
        assert_eq!(true, fileinfo.is_ok());
        assert_eq!(true, fileinfo.unwrap().is_none());
    }

    #[test]
    fn test_fileinfo_constructor() {
        let key = "test_key";
        let key_extractor = TestKeyExtractor {
            key: Some(key.to_string()),
        };

        let fileinfo_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "./dir/test.txt";
        let fileinfo = fileinfo_constructor.gen_fileinfo(
            path::PathBuf::from(filepath.to_string()),
            &vec!["txt".to_string()],
        );
        assert_eq!(true, fileinfo.is_ok());

        let fileinfo = fileinfo.unwrap();
        assert_eq!(true, fileinfo.is_some());

        let fileinfo = fileinfo.unwrap();
        println!("return file info: {:?}", fileinfo);
        assert_eq!(filepath.to_string(), fileinfo.filepath);
        assert_eq!("txt", fileinfo.extension);
        assert_eq!(key, fileinfo.key);
    }
}
