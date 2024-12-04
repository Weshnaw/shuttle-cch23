use std::fs::remove_dir_all;

use anyhow::Context;
use axum::{body::Bytes, response::IntoResponse};
use gix::{revision::walk::Sorting, traverse::{commit::simple::CommitTimeOrder, tree}};
use tar::Archive;
use tracing::{debug, info};

use crate::router::Error;

pub async fn task_01_files(data: Bytes) -> Result<impl IntoResponse, Error> {
    let result = Archive::new(data.as_ref())
        .entries()
        .context("Failed to decrompress bytes")?
        .count();
    info!(?result);
    Ok(result.to_string())
}

pub async fn task_01_size(data: Bytes) -> Result<impl IntoResponse, Error> {
    let result = Archive::new(data.as_ref())
        .entries()
        .context("Failed to decrompress bytes")?
        .map(|file| file.unwrap().header().size().unwrap_or_default())
        .sum::<u64>();
    info!(?result);
    Ok(result.to_string())
}

const TEMP_DIR: &str = "tmp";
pub async fn task_02(data: Bytes) -> Result<impl IntoResponse, Error> {
    let _ = remove_dir_all(TEMP_DIR);
    let mut archive = Archive::new(data.as_ref());
    archive
        .unpack(TEMP_DIR)
        .context("Failed to decrompress bytes")?;
    info!("archive unpacked");
    let repo = gix::discover(TEMP_DIR).context("Failed to find repo")?;

    let commit = repo
        .rev_parse_single("christmas")
        .context("Failed to rev_parse_single")?
        .object()
        .context("Failed to get objects")?
        .try_into_commit()
        .context("Failed to get commit")?;

    let result = repo
        .rev_walk([commit.id])
        .sorting(Sorting::ByCommitTime(CommitTimeOrder::NewestFirst))
        .all()
        .context("Failed to rev_walk")?
        .find_map(|commit| {
            let commit = commit.as_ref().unwrap().object().unwrap();
            let author = commit.author().unwrap().name;
            let id = commit.id().to_string();
            let tree = commit.tree().unwrap();
            let mut rec = tree::Recorder::default();
            tree.traverse().breadthfirst(&mut rec).unwrap();
            debug!("{} {}", author, id);
            if let Some(santa) = rec
                .records
                .into_iter()
                .find(|entry| entry.filepath.ends_with(br#"santa.txt"#))
            {
                info!("\nsanta found: \n{} {}", author, id);
                let oid = santa.oid;
                let obj = repo.find_object(oid).unwrap();
                let data = obj.data.clone();
                let file = String::from_utf8(data).unwrap_or_default();
                info!(?file);
                if file.contains("COOKIE") {
                    Some(format!("{} {}", author, id))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_default();

    remove_dir_all(TEMP_DIR).context("Failed to remove directory")?;
    Ok(result)
}
