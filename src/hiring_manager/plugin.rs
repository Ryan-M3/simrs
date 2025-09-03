use bevy::prelude::*;

use crate::gregslist::{Advert, Gregslist, VacancyDirty};
use crate::gregslist::plugin::gregslist_expiration_system;
use crate::jobs::Job;

pub struct HiringManagerPlugin;

impl Plugin for HiringManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, post_on_gregslist.after(gregslist_expiration_system));
    }
}

pub fn post_on_gregslist(
    mut events: EventReader<VacancyDirty>,
    jobs: Query<&Job>,
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
