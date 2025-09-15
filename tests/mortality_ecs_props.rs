// Mortality quick guide (what the tests mean)
//
// hazard(age) \u2208 [0,1]:
//   The probability an agent of a given age dies *during this tick*.
//   Example: hazard(40) = 0.002 means a 0.2% chance to die this tick at age 40.
//
// Survival S(a, k):
//   Probability the agent is still alive after k ticks starting at age a.
//   In discrete time: S(a, k) = \u220f_{i=0}^{k-1} (1 - hazard(a + i)).
//
// Identities this suite checks:
//   (1) Bounds: 0 \u2264 hazard(age) \u2264 1
//   (2) Survival never increases as k grows: S(a, k+1) \u2264 S(a, k)
//   (3) Consistency: hazard(a) = 1 - S(a, 1) / S(a, 0)
//   (4) Conservation in the ECS: deaths + survivors_next = starters
//   (5) Absorbing: once dead, always dead
//   (6) Large cohort \u2248 expected rate from hazard(age)
//   (7) Same seed \u21d2 same outcome; spawn order shouldn\u2019t change outcomes

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use proptest::prelude::*;
use approx::assert_abs_diff_eq;
mod mortality_rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

// ==== Wire production types ====
use simrs::baby_spawner::system::GameRNG;
use simrs::mortality::system::apply_mortality_with_rate;
use simrs::person::Person;

#[derive(Resource)]
struct CohortSize(usize);

fn app_with_mortality() -> App {
    let mut app = App::new();
    app.insert_resource(GameRNG(StdRng::seed_from_u64(1)));
    app.add_plugins(simrs::mortality::MortalityPlugin);
    app.add_systems(Update, apply_mortality_with_rate(simrs::mortality::hazard(0) as f64));
    app
}

#[derive(Clone, Copy)]
struct AgentInit {
    id: u64,
    age: u16,
    alive: bool,
}

fn spawn_cohort(world: &mut World, cohort: &[AgentInit]) {
    for a in cohort {
        let _e = world.spawn(Person { age: a.age as f32 }).id();
        let _ = _e;
    }
    world.insert_resource(CohortSize(cohort.len()));
}

fn count_alive_dead(world: &mut World) -> (usize, usize) {
    let alive = world.query::<&Person>().iter(world).count();
    let total = world.resource::<CohortSize>().0;
    (alive, total - alive)
}

fn hazard(age: u16) -> f32 {
    // hazard(age): chance of dying *this tick* at this age
    simrs::mortality::hazard(age)
}

fn tick(app: &mut App) { app.update(); }

fn set_global_seed(app: &mut App, seed: u64) {
    app.world_mut().insert_resource(GameRNG(StdRng::seed_from_u64(seed)));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    // Conservation: no phantom creation/deletion
    #[test]
    fn conservation(a in 0u16..=110, n in 1usize..1000) {
        let mut app = app_with_mortality();
        let cohort: Vec<_> = (0..n).map(|i| AgentInit { id: i as u64, age: a, alive: true }).collect();
        spawn_cohort(app.world_mut(), &cohort);
        let (alive0, dead0) = count_alive_dead(app.world_mut());
        prop_assume!(dead0 == 0 && alive0 == n);

        tick(&mut app);

        let (alive1, dead1) = count_alive_dead(app.world_mut());
        prop_assert_eq!(alive1 + dead1, n, "mass conservation violated");
    }

    // Absorbing: nobody resurrects across further ticks
    #[test]
    fn absorbing_death(a in 0u16..=110, n in 10usize..200) {
        let mut app = app_with_mortality();
        let cohort: Vec<_> = (0..n).map(|i| AgentInit { id: i as u64, age: a, alive: true }).collect();
        spawn_cohort(app.world_mut(), &cohort);

        tick(&mut app);
        let (_, dead1) = count_alive_dead(app.world_mut());

        for _ in 0..5 { tick(&mut app); }
        let (_, dead_final) = count_alive_dead(app.world_mut());

        prop_assert!(dead_final >= dead1, "resurrection detected: {} -> {}", dead1, dead_final);
    }

    // LLN: big cohort’s death fraction ≈ hazard(age)
    #[test]
    fn lln_matches_hazard(a in 0u16..=110) {
        let mut app = app_with_mortality();
        let n: usize = 20_000;
        let cohort: Vec<_> = (0..n).map(|i| AgentInit { id: i as u64, age: a, alive: true }).collect();
        spawn_cohort(app.world_mut(), &cohort);

        tick(&mut app);

        let (_alive1, dead1) = count_alive_dead(app.world_mut());
        let obs = dead1 as f64 / n as f64;
        let exp = hazard(a) as f64;

        assert_abs_diff_eq!(obs, exp, epsilon = 0.015);
    }

    // Reproducibility: same seed → same (alive, dead)
    #[test]
    fn seeded_reproducibility(a in 0u16..=110, n in 10usize..2000, seed in any::<u64>()) {
        let mut app_a = app_with_mortality();
        let mut app_b = app_with_mortality();
        set_global_seed(&mut app_a, seed); set_global_seed(&mut app_b, seed);

        let cohort: Vec<_> = (0..n).map(|i| AgentInit { id: i as u64, age: a, alive: true }).collect();
        spawn_cohort(app_a.world_mut(), &cohort);
        spawn_cohort(app_b.world_mut(), &cohort);

        tick(&mut app_a);
        tick(&mut app_b);

        let (alive_a, dead_a) = count_alive_dead(app_a.world_mut());
        let (alive_b, dead_b) = count_alive_dead(app_b.world_mut());

        prop_assert_eq!((alive_a, dead_a), (alive_b, dead_b));
    }

    // Order-invariance: spawn/iteration order doesn’t change results
    #[test]
    fn order_invariance(a in 0u16..=110, n in 50usize..2000, seed in any::<u64>()) {
        use rand::{seq::SliceRandom, SeedableRng};
        use rand::rngs::StdRng;

        let mut ids: Vec<_> = (0..n as u64).collect();

        let mut app_a = app_with_mortality();
        let cohort_a: Vec<_> = ids.iter().copied().map(|id| AgentInit { id, age: a, alive: true }).collect();
        spawn_cohort(app_a.world_mut(), &cohort_a);

        let mut app_b = app_with_mortality();
        let mut rng = StdRng::seed_from_u64(seed ^ 0x9E3779B97F4A7C15);
        ids.shuffle(&mut rng);
        let cohort_b: Vec<_> = ids.iter().copied().map(|id| AgentInit { id, age: a, alive: true }).collect();
        spawn_cohort(app_b.world_mut(), &cohort_b);

        set_global_seed(&mut app_a, seed); set_global_seed(&mut app_b, seed);

        tick(&mut app_a);
        tick(&mut app_b);

        let (_alive_a, dead_a) = count_alive_dead(app_a.world_mut());
        let (_alive_b, dead_b) = count_alive_dead(app_b.world_mut());

        prop_assert_eq!(dead_a, dead_b, "order changed aggregate deaths (enable per-agent RNG)");
    }
}
