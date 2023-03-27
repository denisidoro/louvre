use crate::platform::Platform;
use crate::prelude::*;
use walkdir::DirEntry;

#[derive(Debug)]
pub enum Status {
    ShouldInclude,
    Unsupported,
    InvalidPath,
    NoMetadata,
    Directory,
    Denylist(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    pub name: String,
    pub path: PathBuf,
    pub extensions: Vec<String>,
    pub platform: Platform,
    pub denylist: Option<Vec<String>>,
    pub title_map: Option<HashMap<String, String>>,
    pub launch: String,
}

impl Collection {
    pub fn should_include(&self, entry: &DirEntry) -> Status {
        use Status::*;

        let path = entry.path();

        if path.to_str().is_none() {
            return InvalidPath;
        }

        match entry.metadata() {
            Err(_) => NoMetadata,
            Ok(metadata) => {
                if !metadata.is_file() {
                    Directory
                } else {
                    match entry
                        .path()
                        .extension()
                        .map(|ext| ext.to_string_lossy().to_lowercase())
                    {
                        Some(extension) => {
                            if self.extensions.contains(&extension) {
                                let denylist = self
                                    .denylist
                                    .clone()
                                    .unwrap_or_default()
                                    .iter()
                                    .map(|x| Regex::new(x).expect("invalid regex"))
                                    .collect::<Vec<_>>();

                                for regex in denylist {
                                    if regex.is_match(&path.to_string()) {
                                        return Denylist(regex.to_string());
                                    }
                                }

                                ShouldInclude
                            } else {
                                Unsupported
                            }
                        }
                        _ => NoMetadata,
                    }
                }
            }
        }
    }
}
