use serde_json::{json, Value};

use super::super::protocol::{
    BoundedResidencySpeculationObservation, BoundedResidencySpeculativeKindObservation,
};

pub(super) fn value(observation: BoundedResidencySpeculationObservation) -> Value {
    json!({
        "prefetch": kind(observation.prefetch),
        "read_ahead": kind(observation.read_ahead),
        "write_behind": kind(observation.write_behind),
    })
}

fn kind(observation: BoundedResidencySpeculativeKindObservation) -> Value {
    json!({
        "attempts": observation.attempts,
        "admissions": observation.admissions,
        "denials": observation.denials,
        "completions": observation.completions,
        "peak_frames": observation.peak_frames,
        "terminal_frames": observation.terminal_frames,
        "hits": observation.hits,
        "effectful_misses": observation.effectful_misses,
        "hit_signal_requests": observation.hit_signal_requests,
        "denial_signal_requests": observation.denial_signal_requests,
        "effectful_signal_requests": observation.effectful_signal_requests,
        "signal_family_exact": observation.signal_family_exact,
        "foundational_basis_exact": observation.foundational_basis_exact,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_keeps_zero_signal_and_exact_basis_fields_distinct() {
        let encoded = kind(BoundedResidencySpeculativeKindObservation {
            attempts: 5,
            admissions: 4,
            denials: 1,
            completions: 4,
            peak_frames: 2,
            terminal_frames: 0,
            hits: 1,
            effectful_misses: 3,
            hit_signal_requests: 0,
            denial_signal_requests: 0,
            effectful_signal_requests: 3,
            signal_family_exact: true,
            foundational_basis_exact: true,
        });
        assert_eq!(encoded["hit_signal_requests"], 0);
        assert_eq!(encoded["denial_signal_requests"], 0);
        assert_eq!(encoded["effectful_signal_requests"], 3);
        assert_eq!(encoded["foundational_basis_exact"], true);
    }
}
