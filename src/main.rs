#![feature(unix_chown)]
use error::Result;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::os::unix::fs as unixfs;
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;
use std::process::exit;

mod error;
mod file;
mod key;

fn copy_file<'a>(from: &'a str, to: &'a str) -> Result<'a, u64> {
    fs::copy(from, to)?;

    let meta = fs::metadata(from)?;
    let user_id = meta.uid();
    let group_id = meta.gid();
    println!("uid: {}, gid: {}", user_id, group_id);
    unixfs::chown(to, Some(user_id), Some(group_id))?;

    let perms = fs::metadata(from)?.permissions();
    println!("perm: {:?}", perms);
    fs::set_permissions(to, perms)?;

    Ok(0)
}

fn run(dir: &str) -> Result<i32> {
    let patterns = vec![r"S(\d{2})E(\d{2})".to_string()];
    let key_extractor = key::RegexExtractor::new(patterns)?;
    let fileinfo_constructor = file::FileInfoConstructor::new(&key_extractor);

    let subtitle_extensions = vec!["srt", "ass"];
    let mut subtitle_fileinfo_list = fileinfo_constructor.from_dir(&dir, &subtitle_extensions)?;

    let mut subtitles_map = HashMap::new();
    while subtitle_fileinfo_list.len() > 0 {
        let key = subtitle_fileinfo_list[0].key.clone();
        subtitles_map.insert(key, subtitle_fileinfo_list.swap_remove(0)); // TODO: check existense
    }

    let movie_extensions = vec!["mkv", "mp4"];
    let movie_fileinfo_list = fileinfo_constructor.from_dir(&dir, &movie_extensions)?;

    for ref movie_fileinfo in movie_fileinfo_list {
        if let Some(subtitle_fileinfo) = subtitles_map.get(&movie_fileinfo.key) {
            let mut subtitle_new_path = PathBuf::from(&movie_fileinfo.filepath);
            subtitle_new_path.set_extension(&subtitle_fileinfo.extension);

            println!(
                "{} -> {}\n\tfrom => {}\n\tto => {:?}",
                &movie_fileinfo.filepath,
                movie_fileinfo.key,
                subtitle_fileinfo.filepath,
                subtitle_new_path,
            );

            copy_file(
                &subtitle_fileinfo.filepath,
                subtitle_new_path.to_str().unwrap(),
            )
            .ok();
        }
    }

    return Ok(0);
}

fn main() {
    let dir = env::args().nth(1).expect("missing dir");

    match run(&dir) {
        Ok(_) => println!("done"),
        Err(e) => {
            println!("catch some error: {:?}", e);
            exit(1);
        }
    };
}
