use crate::collection::{self, Collection};
use crate::igdb;
use crate::meta::{self, Meta};
use crate::prelude::*;
use crate::title;
use crate::worker::{self, FileWorker, WorkerResult};
use clap::Args;
use common::deser::to_yaml_str;
use std::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, Args)]
pub struct Input {}

impl FileWorker for Input {
    fn get_filepaths(&self, collections: &[Collection]) -> Result<HashMap<usize, Vec<PathBuf>>> {
        let mut filepaths_map = HashMap::new();

        for (from_index, collection) in collections.iter().enumerate() {
            let collection_path = &collection.path;
            let mut filepaths: Vec<PathBuf> = vec![];

            let entries = WalkDir::new(collection_path)
                .max_depth(6)
                .into_iter()
                .filter_map(|e| e.ok());

            for entry in entries {
                let status = collection.should_include(&entry);
                if let collection::Status::ShouldInclude = status {
                    let filepath = entry.path().strip_prefix(collection_path)?.to_owned();
                    filepaths.push(filepath);
                }
            }

            filepaths.sort();
            filepaths_map.insert(from_index, filepaths);
        }

        Ok(filepaths_map)
    }

    fn process(collection: &Collection, file: &Path, system: &System) -> Result<WorkerResult> {
        let title = title::prettify(file, &collection.title_map);
        // dbg!(&title);
        let igdb_client = system.get::<igdb::Client>()?;
        let was_already_processed = process_title(collection, &title, file, igdb_client)?;
        let progress = if was_already_processed {
            (0, 100)
        } else {
            (100, 0)
        };
        Ok(WorkerResult {
            entry: title,
            progress,
        })
    }
}

fn yaml_path(collection: &Collection, title: &str) -> PathBuf {
    let mut p = collection.path.clone();
    p.push(meta::FOLDER_NAME);
    p.push(&title.replace(": ", " ").replace(" - ", " ").trim());
    p.push(meta::YAML_NAME);
    p
}

fn process_title(
    collection: &Collection,
    title: &str,
    file: &Path,
    igdb_client: &igdb::Client,
) -> Result<bool> {
    let meta_path = yaml_path(collection, title);

    if meta_path.exists() {
        return Ok(true);
    }

    if let Some(parent) = meta_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let igdb = igdb_client.get_metadata(title, collection.platform)?;
    let meta = Meta {
        file: file.into(),
        igdb,
    };

    let meta_str = to_yaml_str(&meta)?;
    fs::write(meta_path, meta_str)?;

    Ok(false)
}

impl Runnable for Input {
    fn run(&self, system: System) -> Result<()> {
        worker::run(self, system)
    }
}

impl HasDeps for Input {
    fn deps(&self) -> HashSet<TypeId> {
        use crate::components::*;
        [*TWITCH, *IGDB].into()
    }
}
