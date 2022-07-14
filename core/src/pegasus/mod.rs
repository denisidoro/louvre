pub mod collection;
pub mod game;
pub mod platform;

use crate::collection::Collection;
use crate::prelude::*;

pub static TXT_NAME: &str = "metadata.pegasus.txt";

impl Collection {
    pub fn pegasus_path(&self) -> PathBuf {
        self.path.join(TXT_NAME)
    }
}
