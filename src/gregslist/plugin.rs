use bevy::prelude::*;

use super::component::{Gregslist, GregslistConfig, VacancyDirty};

pub struct GregslistPlugin {
    expiry_secs: f32,
}

impl GregslistPlugin {
    pub fn new(expiry_secs: f32) -> Self {
        Self { expiry_secs }
    }
}

impl Plugin for GregslistPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gregslist>();
        app.insert_resource(GregslistConfig {
            expiry_secs: self.expiry_secs,
        });
        app.add_event::<VacancyDirty>();
        app.add_systems(Update, gregslist_expiration_system);
    }
}

pub fn gregslist_expiration_system(
    time: Res<Time>,
    config: Res<GregslistConfig>,
    mut board: ResMut<Gregslist>,
    mut dirty: EventWriter<VacancyDirty>,
) {
    let now = time.elapsed_secs();
    let mut expired: Vec<(Entity, usize)> = Vec::new();
    board.ads.retain(|ad| {
        if now - ad.date_posted > config.expiry_secs {
            expired.push((ad.job, ad.role_index));
            false
        } else {
            true
        }
    });
    for (job, role_index) in expired {
        board.index.remove(&(job, role_index));
        dirty.send(VacancyDirty { job });
    }
}
