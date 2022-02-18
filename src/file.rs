use std::fs;

pub fn filter_paths_with_extension(dir: &str, extensions: &Vec<&str>) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut ret = Vec::<String>::new();

    for path in paths {
        let path = path.unwrap().path();
        if let Some(path_extenstion) = path.extension() {
            for &extension in extensions {
                if path_extenstion == extension {
                    ret.push(path.display().to_string());
                    break;
                }
            }
        }
    }

    return ret;
}
