#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod baby_spawner;
pub mod gregslist;
pub mod hiring_manager;
pub mod graph;
pub mod game_events;
pub mod inventory;
pub mod jobs;
pub mod mortality;
pub mod person;
pub mod personality;
pub mod records;
#[cfg(feature = "graphics")]
pub mod view;

pub use baby_spawner::{BabySpawnerConfig, BabySpawnerPlugin};
pub use gregslist::{Gregslist, Advert, GregslistPlugin, GregslistConfig, VacancyDirty};
pub use hiring_manager::HiringManagerPlugin;
pub use jobs::JobsPlugin;
pub use mortality::MortalityPlugin;
pub use records::{RecordsPlugin, VacancyText, VacancyTextPlugin};
