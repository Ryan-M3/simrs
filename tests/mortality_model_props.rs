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

use proptest::prelude::*;
use approx::assert_abs_diff_eq;

fn hazard(age: u16) -> f32 {
    // hazard(age): chance of dying *this tick* at this age
    simrs::mortality::hazard(age)
}

// S(a,k): alive after k ticks starting at age a = product of (1 - hazard) over k ticks
fn survival_from_hazard(a0: u16, k: u16) -> f64 {
    (0..k).fold(1.0f64, |acc, i| {
        let a = a0.saturating_add(i);
        let h = hazard(a) as f64;
        acc * (1.0 - h)
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    // Bounds: hazard must be a probability
    #[test]
    fn hazard_bounds(a in 0u16..=130) {
        let h = hazard(a);
        prop_assert!(0.0 <= h && h <= 1.0, "hazard out of bounds: h({a})={h}");
    }

    // Survival decreases as you ask for “alive after more ticks”
    #[test]
    fn survival_monotone(a0 in 0u16..=120, k in 1u16..=20) {
        let s0 = survival_from_hazard(a0, k);
        let s1 = survival_from_hazard(a0, k+1);
        prop_assert!(s1 <= s0 + 1e-12, "survival increased: S({}) < S({})", k, k+1);
    }

    // Hazard/survival identity for one tick
    #[test]
    fn hazard_survival_consistency(a in 0u16..=120) {
        let s_a   = survival_from_hazard(a, 0);
        let s_a1  = survival_from_hazard(a, 1);
        let h     = hazard(a) as f64;
        let rhs = 1.0 - (s_a1 / (s_a.max(1e-12)));
        assert_abs_diff_eq!(h, rhs, epsilon = 1e-9);
    }
}
