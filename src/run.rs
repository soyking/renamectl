use std::collections::HashMap;
use std::fs;
use std::os::unix::fs as unixfs;
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;

use clap::Parser;

use crate::error::Result;
use crate::file;
use crate::key;

fn copy_file<'a>(from: &'a str, to: &'a str) -> Result<'a, u64> {
    fs::copy(from, to)?;

    let meta = fs::metadata(from)?;
    let user_id = meta.uid();
    let group_id = meta.gid();
    debug!("file: {}, uid: {}, gid: {}", from, user_id, group_id);
    unixfs::chown(to, Some(user_id), Some(group_id))?;

    let perms = fs::metadata(from)?.permissions();
    debug!("file: {}, perm: {:?}", from, perms);
    fs::set_permissions(to, perms)?;

    Ok(0)
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(long)]
    pub dir: String,
    #[arg(long = "pattern", default_values(&[
        r"(?i)S0(\d{1})(?i)E(\d{2})", // S01E01, s01e01
        r"(?i)S(\d{1})(?i)E(\d{2})", // S1E01
        r"(\d{1})(?i)x(\d{2})", // 1x01
        r"(\d{1})(\d{2})", // 101
    ]))]
    pub patterns: Vec<String>,
    #[arg(long = "sub_ext", default_values(&["srt", "ass"]))]
    pub sub_exts: Vec<String>,
    #[arg(long = "ep_ext", default_values(&["mkv", "mp4"]))]
    pub ep_exts: Vec<String>,
}

pub fn run() -> Result<'static, ()> {
    let c = Config::parse();
    debug!("get config: {:?}", c);

    let key_extractor = key::RegexExtractor::new(&c.patterns)?;
    let file_info_constructor = file::FileInfoConstructor::new(&key_extractor);

    let mut subtitle_file_info_list = file_info_constructor.from_dir(&c.dir, &c.sub_exts)?;
    info!("subtitle file info: {:?}", subtitle_file_info_list);

    let episode_file_info_list = file_info_constructor.from_dir(&c.dir, &c.ep_exts)?;
    info!("episode file info: {:?}", episode_file_info_list);

    let mut subtitles_map = HashMap::new();
    while let Some(file_info) = subtitle_file_info_list.pop() {
        let key = file_info.key.clone();
        let subtitle_file_info_list = subtitles_map.entry(key).or_insert(vec![]);
        subtitle_file_info_list.push(file_info);
    }

    for ref episode_file_info in episode_file_info_list {
        if let Some(subtitle_file_info_list) = subtitles_map.get(&episode_file_info.key) {
            for subtitle_file_info in subtitle_file_info_list {
                let mut subtitle_new_path = PathBuf::from(&episode_file_info.filepath);
                subtitle_new_path.set_extension(&subtitle_file_info.extension);
                let subtitle_new_path = subtitle_new_path.to_str().unwrap();

                if subtitle_new_path == subtitle_file_info.filepath.as_str() {
                    debug!("exclude existed file info: {:?}", subtitle_file_info);

                    continue;
                }

                info!(
                    "\n[episode]: {}, key: {}\n\t{}\n\t↓\n\t{}",
                    episode_file_info.filepath,
                    episode_file_info.key,
                    subtitle_file_info.filepath,
                    subtitle_new_path,
                );

                copy_file(&subtitle_file_info.filepath, subtitle_new_path).unwrap();

                break;
            }
        }
    }

    return Ok(());
}
