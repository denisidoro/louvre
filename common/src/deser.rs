use crate::prelude::*;
use serde::de::DeserializeOwned;

#[cfg(feature = "yaml")]
pub fn yaml_from_path<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: T = serde_yaml::from_reader(reader)?; // TODO: show path + original error message?
    Ok(config)
}

#[cfg(feature = "json")]
pub fn json_from_path<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: T = serde_json::from_reader(reader)?; // TODO: show path + original error message?
    Ok(config)
}

#[cfg(feature = "yaml")]
pub fn yaml_from_str<T>(text: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    serde_yaml::from_str(text)
        .with_context(|| format!("Failed to deserialize into yaml:\n{}", text))
}

#[cfg(feature = "json")]
pub fn json_from_str<T>(text: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(text)
        .with_context(|| format!("Failed to deserialize into json:\n{}", text))
}

#[cfg(feature = "yaml")]
pub fn to_yaml_str<T>(t: &T) -> Result<String>
where
    T: Serialize,
{
    serde_yaml::to_string(t).with_context(|| "Failed to serialize into yaml") // TODO: debug struct?
}
