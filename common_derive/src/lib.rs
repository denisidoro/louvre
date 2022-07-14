#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]

mod deps;
mod runnable;

use synstructure::decl_derive;

decl_derive!([Runnable] => runnable::derive_runnable);
decl_derive!([HasDeps] => deps::derive_has_deps);
