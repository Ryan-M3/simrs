use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::{Time, Real};

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

fn gregslist_expiration_system(
    time: Res<Time<Real>>,
    cfg: Res<GregslistConfig>,
    mut board: ResMut<Gregslist>,
    mut dirty: EventWriter<VacancyDirty>,
) {
    let now = time.elapsed_secs();
    let mut removed: Vec<(Entity, usize)> = Vec::new();
    board.ads.retain(|ad| {
        let keep = now - ad.date_posted <= cfg.expiry_secs;
        if !keep {
            removed.push((ad.job, ad.role_index));
        }
        keep
    });

    for pair in removed {
        board.index.remove(&pair);
        dirty.write(VacancyDirty { job: pair.0 });
    }
}
