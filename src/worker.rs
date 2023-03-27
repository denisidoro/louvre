use crate::collection::Collection;
use crate::prelude::*;
use std::sync::mpsc;
use workerpool::thunk::{Thunk, ThunkWorker};
use workerpool::Pool;

pub type Progress = (u8, u8);

pub struct WorkerResult {
    pub entry: String,
    pub progress: Progress,
}

struct FullWorkerResult {
    collection: String,
    error: Option<Error>,
    res: WorkerResult,
}

pub trait FileWorker: Runnable {
    fn get_filepaths(&self, collections: &[Collection]) -> Result<HashMap<usize, Vec<PathBuf>>>;
    fn process(collection: &Collection, file: &Path, system: &System) -> Result<WorkerResult>;
}

pub fn run<I: FileWorker>(input: &I, system: System) -> Result<()> {
    let system_arc = Arc::new(system);

    let collections = system_arc.config.yaml.collections.clone();
    let n_workers = system_arc.config.yaml.parallelism.workers;

    let filepaths = input.get_filepaths(&collections)?;
    let n_files = filepaths.values().map(|vs| vs.len()).sum();

    if n_files == 0 {
        return Ok(());
    }

    let pool: Pool<ThunkWorker<FullWorkerResult>> = Pool::new(n_workers);

    let (tx, rx) = mpsc::channel();

    for (collection_index, files) in filepaths {
        for file in files {
            let collection = collections
                .get(collection_index)
                .expect("invalid collection index")
                .clone();

            let new_system_arc = Arc::clone(&system_arc);

            pool.execute_to(
                tx.clone(),
                Thunk::of(move || {
                    let collection_name = collection.name.clone();
                    match I::process(&collection, &file, new_system_arc.as_ref()) {
                        Ok(res) => FullWorkerResult {
                            collection: collection_name,
                            res,
                            error: None,
                        },
                        Err(e) => {
                            if e.to_string().contains("Too Many Requests") {
                                panic!("aborting because of too many requests")
                            }
                            FullWorkerResult {
                                collection: collection_name,
                                res: WorkerResult {
                                    entry: file.to_string(),
                                    progress: (0, 0),
                                },
                                error: Some(e),
                            }
                        }
                    }
                }),
            )
        }
    }

    let mut errors = 0;
    let results = rx.iter().take(n_files);

    for result in results {
        let collection = result.collection;
        let error = result.error;
        let entry = result.res.entry;

        let (processed, already_processed) = result.res.progress;
        let original_sum = processed + already_processed;
        let sum = if original_sum == 99 {
            100
        } else {
            original_sum
        };
        let progress = if processed > 0 {
            format!("{}% (+{})", sum, sum - already_processed)
        } else {
            format!("{}%", sum)
        };

        if let Some(e) = error {
            errors += 1;
            info!(collection, entry, error = e.to_string());
        } else {
            info!(progress, collection, entry);
        }
    }

    if errors > 0 {
        Err(anyhow!("{} files failed", errors))
    } else {
        Ok(())
    }
}
