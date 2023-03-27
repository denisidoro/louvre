use super::platform;
use crate::collection::Collection;
use crate::prelude::*;

pub fn to_str(collection: &Collection, files: &[PathBuf]) -> String {
    let files_str = files
        .iter()
        .map(|f| format!("../{}", f.to_string()))
        .collect::<Vec<_>>()
        .join("\n   ");

    format!(
        "collection: {}
launch: {}
shortname: {}
files:
   {}

",
        collection.name,
        collection.launch,
        platform::id(&collection.platform).unwrap_or_default(),
        files_str,
    )
}
