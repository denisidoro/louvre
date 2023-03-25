use crate::collection::Collection;
use crate::igdb;
use crate::meta;
use crate::prelude::*;
use dns_common::deser::yaml_from_path;
use walkdir::WalkDir;

pub static FOLDER_NAME: &str = "meta";
pub static YAML_NAME: &str = "meta.yaml";

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub file: PathBuf,
    pub igdb: igdb::Game,
}

pub fn get_filepaths(collections: &[Collection]) -> Result<HashMap<usize, Vec<PathBuf>>> {
    let mut filepaths_map = HashMap::new();

    for (from_index, collection) in collections.iter().enumerate() {
        let collection_path = &collection.path;

        let mut filepaths: Vec<PathBuf> = vec![];

        let entries = WalkDir::new(&collection_path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok());

        for entry in entries {
            if entry.metadata()?.is_file() {
                let path = entry.path();
                let filename = path
                    .file_name()
                    .context("no filename")?
                    .to_string_lossy()
                    .to_string();

                if filename.ends_with(meta::YAML_NAME) {
                    filepaths.push(path.into())
                }
            }
        }

        filepaths.sort();
        filepaths_map.insert(from_index, filepaths);
    }

    Ok(filepaths_map)
}

pub fn get(path: &Path) -> Result<Meta> {
    yaml_from_path(path)
}
