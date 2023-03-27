pub mod collection;
pub mod game;
pub mod platform;

use crate::collection::Collection;
use crate::{meta, prelude::*};

pub static TXT_NAME: &str = "metadata.pegasus.txt";

impl Collection {
    pub fn pegasus_path(&self) -> PathBuf {
        let mut p = self.path.clone();
        p.push(meta::FOLDER_NAME);
        p.push(TXT_NAME);
        p
    }
}
