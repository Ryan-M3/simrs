use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

use crate::gregslist::component::{Gregslist, VacancyDirty, Advert};
use crate::jobs::component::{Job, Constraint};
use crate::hiring_manager::component::{ApplicationInbox, Resume, HiringConfig, Unemployed, Age};

pub struct HiringManagerPlugin {
    max_hires_per_role_per_cycle: u32,
}

impl HiringManagerPlugin {
    pub fn new(max_hires_per_role_per_cycle: u32) -> Self {
        Self { max_hires_per_role_per_cycle }
    }
}

impl Plugin for HiringManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HiringConfig { max_hires_per_role_per_cycle: self.max_hires_per_role_per_cycle })
            .init_resource::<ApplicationInbox>()
            .add_systems(Startup, mark_jobs_dirty_on_startup)
            .add_systems(Update, (post_job_openings, apply_for_jobs, evaluate_and_assign).chain());
    }
}

// Seed initial postings
fn mark_jobs_dirty_on_startup(
    jobs: Query<Entity, With<Job>>,
    mut dirty: EventWriter<VacancyDirty>,
) {
    for job in jobs.iter() {
        dirty.write(VacancyDirty { job });
    }
}

// Post/remove adverts so Gregslist reflects current vacancies for dirty jobs.
fn post_job_openings(
    time: Res<Time>,
    mut board: ResMut<Gregslist>,
    mut dirty_events: EventReader<VacancyDirty>,
    jobs: Query<&Job>,
) {
    let now = time.elapsed_secs();

    for ev in dirty_events.read() {
        if let Ok(job_data) = jobs.get(ev.job) {
            // Reconcile each role of this job.
            for (i, (spec, members)) in job_data.roles.iter().enumerate() {
                let needed = spec.min.saturating_sub(members.len() as u32);
                let key = (ev.job, i);

                if needed > 0 {
                    // ensure one advert exists
                    if !board.index.contains(&key) {
                        board.ads.push(Advert { job: ev.job, role_index: i, date_posted: now });
                        board.index.insert(key);
                    }
                } else {
                    // remove any existing advert for a now-filled role
                    if board.index.remove(&key) {
                        board.ads.retain(|ad| !(ad.job == ev.job && ad.role_index == i));
                    }
                }
            }
        }
    }
}

// Unemployed agents apply to adverts that they satisfy by constraints.
fn apply_for_jobs(
    board: Res<Gregslist>,
    jobs: Query<&Job>,
    ages: Query<&Age>,
    applicants: Query<Entity, With<Unemployed>>,
    mut inbox: ResMut<ApplicationInbox>,
) {
    // Naive v1: apply once per frame to all matching adverts (inbox is drained before next frame)
    for applicant in applicants.iter() {
        let age = ages.get(applicant).ok().map(|a| a.years);
        for ad in board.ads.iter() {
            if let Ok(job) = jobs.get(ad.job) {
                if let Some((spec, members)) = job.roles.get(ad.role_index) {
                    // must not already be a member
                    if members.iter().any(|&e| e == applicant) {
                        continue;
                    }

                    // constraints check (age only for now)
                    if constraints_ok(spec, age) {
                        inbox.resumes.push(Resume {
                            applicant,
                            job: ad.job,
                            role_index: ad.role_index,
                        });
                    }
                }
            }
        }
    }
}

// Hire up to available capacity and configured batch size; remove Unemployed on success.
fn evaluate_and_assign(
    mut inbox: ResMut<ApplicationInbox>,
    mut jobs: Query<&mut Job>,
    mut commands: Commands,
    cfg: Res<HiringConfig>,
    mut dirty: EventWriter<VacancyDirty>,
) {
    if inbox.resumes.is_empty() {
        return;
    }

    // Group applications by (job, role_index) to count capacity once
    // (simple single-pass approach: we’ll compute on demand per resume)
    let resumes = std::mem::take(&mut inbox.resumes);

    for r in resumes {
        if let Ok(mut job) = jobs.get_mut(r.job) {
            if let Some((spec, members)) = job.roles.get_mut(r.role_index) {
                // capacity left = min(max - current, batch, min - current)
                let current = members.len() as u32;
                let cap_max = spec.max.saturating_sub(current);
                let cap_min = spec.min.saturating_sub(current);
                let batch_cap = cfg.max_hires_per_role_per_cycle;

                // If already at or above max, skip
                if cap_max == 0 {
                    continue;
                }

                // Hire now if we still need seats (prefer filling to min first).
                // We admit within batch cap by checking how many we’ve added this frame.
                // Simple guard: only push if still below both min/max bound.
                let still_below_min = cap_min > 0;
                let still_below_max = cap_max > 0;

                if still_below_min || still_below_max {
                    // Prevent duplicate membership
                    if !members.iter().any(|&e| e == r.applicant) {
                        // Count how many we already added this frame to this role
                        // (approx by checking members length delta after each push)
                        if members.len() as u32 - current < batch_cap {
                            members.push(r.applicant);
                            commands.entity(r.applicant).remove::<Unemployed>();
                            dirty.write(VacancyDirty { job: r.job });
                        }
                    }
                }
            }
        }
    }
}

// --- helpers ---
fn constraints_ok(spec: &crate::jobs::component::RoleSpec, maybe_age: Option<u8>) -> bool {
    use Constraint::*;
    for c in &spec.constraints {
        match *c {
            AgeLessThan(n) => {
                if let Some(age) = maybe_age {
                    if !(age < n) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            AgeAtLeast(n) => {
                if let Some(age) = maybe_age {
                    if !(age >= n) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    true
}
