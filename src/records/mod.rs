pub mod plugin;
pub mod records;
pub mod rolling_mean;
#[cfg(feature = "graphics")]
pub mod ui;

pub use self::records::{record_births, record_deaths, record_employment_rate, Records};
pub use self::rolling_mean::RollingMean;
#[cfg(feature = "graphics")]
pub use self::ui::{
    spawn_employment_text,
    spawn_population_text,
    spawn_vacancy_text,
    update_employment_text,
    update_population_text,
    update_vacancy_text,
    VacancyText,
    VacancyTextPlugin,
};
pub use plugin::RecordsPlugin;
