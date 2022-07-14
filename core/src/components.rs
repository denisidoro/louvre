use crate::igdb;
use crate::prelude::*;
use crate::twitch;
use once_cell::sync::Lazy;

pub static TWITCH: Lazy<TypeId> = Lazy::new(TypeId::of::<twitch::Client>);
pub static IGDB: Lazy<TypeId> = Lazy::new(TypeId::of::<igdb::Client>);

static DEP_ERROR: &str = "dependency not initialized";

pub fn init(system: &mut System) -> Result<()> {
    let twitch = system.maybe_add(&*TWITCH, |s| twitch::Client::new(&s.config))?;

    let _igdb = system.maybe_add(&*IGDB, |s| {
        let twitch = twitch.context(DEP_ERROR)?;
        igdb::Client::new(Arc::clone(&s.config), &twitch)
    })?;

    Ok(())
}
