use anyhow::Context;

use crate::error::Result;
use crate::key;
use std::fs;
use std::path;

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

    pub fn from_dir(&self, dir: &str, extensions: &Vec<&str>) -> Result<Vec<FileInfo>> {
        let paths = fs::read_dir(dir).with_context(|| format!("read dir {}", dir))?;

        return self.from_paths(paths, extensions);
    }

    fn from_paths(&self, paths: fs::ReadDir, extensions: &Vec<&str>) -> Result<Vec<FileInfo>> {
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
        extensions: &Vec<&str>,
    ) -> Result<Option<FileInfo>> {
        if let Some(filepath_extenstion) = filepath.extension() {
            for &extension in extensions {
                if filepath_extenstion == extension {
                    if let Some(filepath_str) = filepath.to_str() {
                        if let Some(key) = self.key_extractor.extract(filepath_str) {
                            return Ok(Some(FileInfo {
                                filepath: filepath.display().to_string(),
                                extension: extension.to_string(),
                                key,
                            }));
                        } else {
                            // debug
                            println!("skip filepath: {:?}", filepath_str)
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::FileInfoConstructor;
    use crate::key::Extractor;
    use std::{path, vec};

    #[test]
    fn test_fileinfo_constructor() {
        struct TestKeyExtractor {
            key: String,
        }
        impl Extractor for TestKeyExtractor {
            fn extract(&self, _: &str) -> Option<String> {
                return Some(self.key.clone());
            }
        }

        let key = "test_key";
        let key_extractor = TestKeyExtractor {
            key: key.to_string(),
        };

        let fileinfo_constructor = FileInfoConstructor::new(&key_extractor);

        let filepath = "./dir/test.txt";
        let fileinfo = fileinfo_constructor
            .gen_fileinfo(path::PathBuf::from(filepath.to_string()), &vec!["txt"]);
        assert_eq!(true, fileinfo.is_ok());

        let fileinfo = fileinfo.unwrap();
        assert_eq!(true, fileinfo.is_some());

        let fileinfo = fileinfo.unwrap();
        assert_eq!(filepath.to_string(), fileinfo.filepath);
        assert_eq!("txt", fileinfo.extension);
        assert_eq!(key, fileinfo.key);
    }
}
