use crate::error::Result;
use crate::file;
use crate::key;
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs as unixfs;
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;

fn copy_file<'a>(from: &'a str, to: &'a str) -> Result<'a, u64> {
    fs::copy(from, to)?;

    let meta = fs::metadata(from)?;
    let user_id = meta.uid();
    let group_id = meta.gid();
    debug!("uid: {}, gid: {}", user_id, group_id);
    unixfs::chown(to, Some(user_id), Some(group_id))?;

    let perms = fs::metadata(from)?.permissions();
    debug!("perm: {:?}", perms);
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
    pub episode_exts: Vec<String>,
}

pub fn run(c: &Config) -> Result<i32> {
    let key_extractor = key::RegexExtractor::new(&c.patterns)?;
    let fileinfo_constructor = file::FileInfoConstructor::new(&key_extractor);

    let mut subtitle_fileinfo_list = fileinfo_constructor.from_dir(&c.dir, &c.sub_exts)?;
    debug!("subtitle files: {:?}", subtitle_fileinfo_list);

    let mut subtitles_map = HashMap::new();
    while subtitle_fileinfo_list.len() > 0 {
        let key = subtitle_fileinfo_list[0].key.clone();
        subtitles_map.insert(key, subtitle_fileinfo_list.swap_remove(0)); // TODO: check existense
    }

    let movie_fileinfo_list = fileinfo_constructor.from_dir(&c.dir, &c.episode_exts)?;
    debug!("movie files: {:?}", movie_fileinfo_list);

    for ref movie_fileinfo in movie_fileinfo_list {
        if let Some(subtitle_fileinfo) = subtitles_map.get(&movie_fileinfo.key) {
            let mut subtitle_new_path = PathBuf::from(&movie_fileinfo.filepath);
            subtitle_new_path.set_extension(&subtitle_fileinfo.extension);

            info!(
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
