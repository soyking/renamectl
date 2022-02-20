use crate::error::{Error, Result};
use crate::key;
use std::fs;
use std::path;

pub struct FileInfo {
    pub filepath: String,
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
        let paths = match fs::read_dir(dir) {
            Ok(paths) => paths,
            Err(e) => return Err(Error::from_string(&e, "read dir")),
        };

        return self.from_paths(paths, extensions);
    }

    fn from_paths(&self, paths: fs::ReadDir, extensions: &Vec<&str>) -> Result<Vec<FileInfo>> {
        let mut ret = Vec::<FileInfo>::new();
        for path in paths {
            let filepath = match path {
                Ok(path) => path.path(),
                Err(e) => return Err(Error::from_string(&e, "get path")),
            };

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
                        let key = self.key_extractor.extract(filepath_str)?;

                        return Ok(Some(FileInfo {
                            filepath: filepath.display().to_string(),
                            extension: extension.to_string(),
                            key,
                        }));
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
    use crate::error::Result;
    use crate::key::Extractor;
    use std::path;

    #[test]
    fn test_fileinfo_constructor() {
        struct TestKeyExtractor {
            key: String,
        }
        impl Extractor for TestKeyExtractor {
            fn extract(&self, _: &str) -> Result<String> {
                return Ok(self.key.clone());
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
