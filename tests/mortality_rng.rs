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

use rand08::{Rng, SeedableRng};
use rand08::rngs::StdRng;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Per-agent RNG: same (seed, agent_id, tick) → same draw.
// Prevents iteration order from deciding who dies.
pub fn draw_u01(global_seed: u64, agent_id: u64, tick: u64) -> f32 {
    let mut h = DefaultHasher::new();
    global_seed.hash(&mut h);
    agent_id.hash(&mut h);
    tick.hash(&mut h);
    let seed = h.finish();
    let mut rng = StdRng::seed_from_u64(seed);
    rng.r#gen::<f32>() // in [0,1)
}
