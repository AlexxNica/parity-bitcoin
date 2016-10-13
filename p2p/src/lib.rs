#[macro_use]
extern crate futures;
extern crate futures_cpupool;
extern crate rand;
extern crate time;
extern crate tokio_core;
extern crate parking_lot;

extern crate bitcrypto as crypto;
extern crate message;
extern crate primitives;
extern crate serialization as ser;

pub mod io;
pub mod net;
pub mod util;
mod config;
mod event_loop;
mod p2p;
mod run;

pub const VERSION: u32 = 70_001;
pub const USER_AGENT: &'static str = "pbtc";

pub use primitives::{hash, bytes};

pub use config::Config;
pub use event_loop::{event_loop, forever};
pub use run::run;
pub use p2p::P2P;

pub type PeerId = usize;

