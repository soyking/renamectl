mod file;
mod key;
use std::env;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dir = env::args().nth(1).expect("missing dir");

    let patterns = vec![r"S(\d{2})E(\d{2})"];

    let subtitle_extensions = vec!["srt", "ass"];
    let subtitle_filenames = file::filter_paths_with_extension(&dir, &subtitle_extensions);
    // println!("{:?}", subtitle_filenames);

    let mut subtitle_keys = HashMap::new();
    for subtitle_filename in subtitle_filenames {
        let subtitle_key = key::extract_from_patterns(&subtitle_filename, &patterns);
        // println!("{:?} => {:?}", subtitle_filename, subtitle_key);
        subtitle_keys.insert(subtitle_key.to_string(), subtitle_filename.to_string());
    }

    let movie_extensions = vec!["mkv", "mp4"];
    let movie_filenames = file::filter_paths_with_extension(&dir, &movie_extensions);
    // println!("{:?}", movie_filenames);
    for movie_filename in movie_filenames {
        let movie_key = key::extract_from_patterns(&movie_filename, &patterns);
        // println!("{:?} => {:?}", movie_filename, movie_key);
        if let Some(subtitle_filename) = subtitle_keys.get(&movie_key) {
            // println!("{:?} => {:?}", movie_filename, subtitle_filename);
            let subtitle_path = PathBuf::from(subtitle_filename);
            let mut subtitle_new_path = PathBuf::from(&movie_filename);
            if let Some(subtitle_extention) = subtitle_path.extension() {
                subtitle_new_path.set_extension(subtitle_extention);
                println!(
                    "{}\n{}\n{:?}\n{:?}",
                    &movie_filename, movie_key, &subtitle_path, subtitle_new_path
                );
                fs::copy(subtitle_path, subtitle_new_path).ok();
            }
        }
    }
}
