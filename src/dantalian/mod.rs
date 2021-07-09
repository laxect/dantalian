use crate::{
    bangumi::{get_anime_data, search_anime, SubjectBaseWithNum},
    error, info,
    nfogen::{Generator, TVSHOW_NFO_NAME},
};
use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use config::Config;
use data::AnimeData;
use job::Job;
use std::{collections::HashSet, fs::File, io::Write, path::Path};
use utils::path_str;
use walkdir::WalkDir;

use self::utils::is_video_file;

mod config;
mod data;
mod job;
mod utils;

pub async fn dantalian(source: &Path, forces: &HashSet<String>) -> Result<()> {
    info!("Run dantalian for {}", source.to_string_lossy());
    for e in WalkDir::new(source).min_depth(1).max_depth(1) {
        let entry = e?;
        if entry.file_type().is_dir() {
            let path = path_str(entry.path())?;
            info!(ind: 1, "Check {} ...", path);
            match handle_dir(entry.path(), forces.contains(path)).await {
                Ok(_) => info!(ind: 2, "Completed!"),
                Err(e) => error!(ind: 2, "Failed: {}", e),
            };
        }
    }
    Ok(())
}

async fn handle_dir(path: &Path, force: bool) -> Result<()> {
    let config = Config::parse(path).await?;
    let job = Job::parse(path, &config, force)?;
    if job.is_empty() {
        info!(ind: 3, "No file should be generate, skip.");
        return Ok(());
    }
    let bgm_data = get_anime_data(job.subject_id).await.with_context(|| "get_anime_data")?;
    info!(ind: 3,
        "Fetch anime data for: [{}] {} / {}",
        &bgm_data.subject.id,
        &bgm_data.subject.name,
        &bgm_data.subject.name_cn
    );
    let anime_data = AnimeData::from(bgm_data);
    let generator = Generator::new();
    if job.should_gen_tvshow {
        info!(ind: 4, "Generate {} ...", TVSHOW_NFO_NAME);
        let file_str = generator.gen_tvshow_nfo(&anime_data.tvshow)?;
        let file_path = Path::new(path).join(TVSHOW_NFO_NAME);
        let mut f = File::create(file_path)?;
        f.write_all(&file_str.into_bytes())?;
    }
    for episode in job.episodes {
        info!(ind: 4, "Generate {} ...", &episode.filename);
        let data = anime_data
            .find_episode(&episode.index, episode.is_sp)
            .ok_or_else(|| anyhow!("Can't find ep {}, is_sp {}", episode.index, episode.is_sp))?;
        let file_str = generator.gen_episode_nfo(data)?;
        let mut f = File::create(&episode.filename)?;
        f.write_all(&file_str.into_bytes())?;
    }
    Ok(())
}

const PLACEHOLDER: &str = "(?P<sp>.*)";
pub async fn generate_config(keywords: Vec<String>, path: &Path) -> Result<()> {
    let keyword = keywords.concat();
    let res = search_anime(&keyword).await?;
    if res.list.len() > 15 {
        error!(ind: 2, "found too many results! please try again with more clear keyword");
        return Ok(());
    }
    for (ind, item) in res.list.iter().enumerate() {
        let item_with_num = SubjectBaseWithNum { num: ind, inner: item };
        info!("{:>1}", item_with_num);
    }
    let mut buf = String::new();
    print!("\n  choose the one is right: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut buf)?;
    if let Ok(num) = buf.trim().parse::<usize>() {
        if let Some(item) = res.list.get(num) {
            let filenames_str = gather_media_filename(path)?;
            let filenames: Vec<&str> = filenames_str.iter().map(AsRef::as_ref).collect();
            let re = if filenames.is_empty() {
                let name_qry = format!("{}|{}", item.name, item.name_cn);
                config::default_ep_regex(&name_qry)?
            } else if filenames.len() >= 2 {
                regex_gather::gather_regex(&filenames, PLACEHOLDER, true)?
            } else {
                let patterns = regex_gather::guess_pattern_with_escape(&filenames[0], PLACEHOLDER, regex::escape)?;
                for (ind, item) in patterns.iter().enumerate() {
                    println!(
                        "  {:>2} - {}\n     - {}{}{}\n",
                        ind,
                        item.pattern,
                        item.preview_left,
                        item.highlight.bold().green(),
                        item.preview_right
                    );
                }
                print!("\n  choose the regex is right: ");
                std::io::stdout().flush()?;
                buf.clear();
                std::io::stdin().read_line(&mut buf)?;
                if let Some(choose) = patterns.get(buf.trim().parse::<usize>()?) {
                    regex::Regex::new(&choose.pattern)?
                } else {
                    return Err(anyhow::anyhow!("not a valid number"));
                }
            };
            let config = Config {
                subject_id: item.id,
                episode_re: re,
            };
            config.save_to_dir(path)?;
            return Ok(());
        }
    }
    error!(ind: 2, "not a valid number!");
    Ok(())
}

pub fn gather_media_filename(path: &Path) -> Result<Vec<String>> {
    let mut res = Vec::new();
    for e in WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry = e?;
        if entry.file_type().is_file() {
            if is_video_file(entry.path()) {
                res.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    Ok(res)
}
