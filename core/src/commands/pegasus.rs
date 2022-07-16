use crate::meta;
use crate::pegasus;
use crate::prelude::*;
use clap::Args;
use clap::Subcommand;
use std::fs;
use std::sync::Arc;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    Gen,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

impl Input {
    fn handle_init(&self, system: System) -> Result<()> {
        let system = Arc::new(system);

        let collections = &system.config.yaml.collections;
        let get_collection = |i: usize| collections.get(i).expect("invalid collection index");

        let filepath_map = meta::get_filepaths(collections)?;

        for (collection_index, meta_files) in filepath_map {
            let mut games_buffer = String::new();
            let mut files = vec![];

            let collection = get_collection(collection_index);

            for meta_file in meta_files {
                let meta = meta::get(&meta_file)?;
                let file = meta.file.clone();

                if !collection.path.join(&file).exists() {
                    continue;
                }

                files.push(file);
                games_buffer.push_str(&pegasus::game::to_str(meta, &meta_file, collection)?);
                games_buffer.push_str("\n\n");
            }

            let txt = format!(
                "{}\n{}",
                pegasus::collection::to_str(collection, &files),
                games_buffer
            );
            let pegasus_path = collection.pegasus_path();
            fs::write(pegasus_path, txt)?;
        }

        Ok(())
    }
}

impl Runnable for Input {
    fn run(&self, system: System) -> Result<()> {
        match self.subcmd {
            SubCmd::Gen => self.handle_init(system),
        }
    }
}

impl HasDeps for Input {}
