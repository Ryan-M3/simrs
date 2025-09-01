pub mod plugin;
pub mod records;
pub mod rolling_mean;
pub mod ui;

pub use self::records::{record_births, record_deaths, Records};
pub use self::rolling_mean::RollingMean;
pub use self::ui::{spawn_population_text, update_population_text};
pub use plugin::RecordsPlugin;
