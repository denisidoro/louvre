use crate::collection::Collection;
use crate::igdb;
use crate::igdb::Game;
use crate::meta;
use crate::prelude::*;

use crate::worker::{self, FileWorker, WorkerResult};
use clap::Args;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    Download,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

impl FileWorker for Input {
    fn get_filepaths(&self, collections: &[Collection]) -> Result<HashMap<usize, Vec<PathBuf>>> {
        meta::get_filepaths(collections)
    }

    fn process(collection: &Collection, file: &Path, system: &System) -> Result<WorkerResult> {
        let game = meta::get(file)?.igdb;
        let title = get_title(file)?;

        let progress = process_game(collection, &title, &game, system.get::<igdb::Client>()?)?;
        Ok(WorkerResult {
            entry: title,
            progress,
        })
    }
}

fn get_title(file: &Path) -> Result<String> {
    Ok(file
        .parent()
        .context("no parent")?
        .file_name()
        .context("no filename")?
        .to_string())
}

fn process_game(
    collection: &Collection,
    title: &str,
    game: &Game,
    igdb_client: &igdb::Client,
) -> Result<(u8, u8)> {
    let game_path = {
        let mut p = collection.path.to_owned();
        p.push(meta::FOLDER_NAME);
        p.push(title);
        p
    };

    let mut already_processed = 0;
    let mut processed = 0;
    let delta = 33;

    let image_definitions = vec![
        (
            game.cover.as_ref(),
            "boxFront.jpg",
            igdb::ImageSize::CoverBig,
        ),
        (
            game.artworks.as_ref().and_then(|artworks| artworks.get(0)),
            "background.jpg",
            igdb::ImageSize::ScreenshotBig,
        ),
        (
            game.screenshots
                .as_ref()
                .and_then(|artworks| artworks.get(0)),
            "screenshot.jpg",
            igdb::ImageSize::ScreenshotBig,
        ),
    ];

    for (image_option, image_name, image_size) in image_definitions {
        if let Some(image) = image_option {
            let image_path = game_path.join(image_name);
            if image_path.exists() {
                already_processed += delta;
            } else {
                processed += delta;
                igdb_client.download(image, image_size, true, &image_path)?;
            }
        }
    }

    Ok((processed, already_processed))
}

impl Runnable for Input {
    fn run(&self, system: System) -> Result<()> {
        match self.subcmd {
            SubCmd::Download => worker::run(self, system),
        }
    }
}

impl HasDeps for Input {
    fn deps(&self) -> HashSet<TypeId> {
        use crate::components::*;
        [*TWITCH, *IGDB].into()
    }
}
