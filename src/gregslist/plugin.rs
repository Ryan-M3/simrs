use bevy::prelude::*;

use super::component::{Advert, Gregslist, GregslistConfig, VacancyDirty};

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
        app.add_systems(Update, (gregslist_cleaner, hiring_manager_post).chain());
    }
}

fn gregslist_cleaner(
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

fn hiring_manager_post(
    mut events: EventReader<VacancyDirty>,
    jobs: Query<&crate::jobs::component::Job>,
    mut board: ResMut<Gregslist>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    for ev in events.read() {
        if let Ok(job) = jobs.get(ev.job) {
            for (i, (spec, members)) in job.roles.iter().enumerate() {
                let vacancy = spec.min.saturating_sub(members.len() as u32);
                let key = (ev.job, i);
                if vacancy > 0 {
                    if !board.index.contains(&key) {
                        board.ads.push(Advert {
                            job: ev.job,
                            role_index: i,
                            date_posted: now,
                        });
                        board.index.insert(key);
                    }
                } else if board.index.remove(&key) {
                    board
                        .ads
                        .retain(|ad| !(ad.job == ev.job && ad.role_index == i));
                }
            }
        }
    }
}
