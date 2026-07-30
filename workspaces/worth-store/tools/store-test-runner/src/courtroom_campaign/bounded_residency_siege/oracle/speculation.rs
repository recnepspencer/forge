use super::super::protocol::{
    BoundedResidencySpeculationObservation, BoundedResidencySpeculativeKindObservation,
};

pub(super) fn verify(observation: BoundedResidencySpeculationObservation) -> Result<(), String> {
    verify_kind(
        "prefetch",
        observation.prefetch,
        ExpectedKind {
            attempts: 5,
            admissions: 4,
            completions: 4,
            peak_frames: 2,
            hits: 1,
            effectful_misses: 3,
        },
    )?;
    verify_kind(
        "read-ahead",
        observation.read_ahead,
        ExpectedKind {
            attempts: 3,
            admissions: 2,
            completions: 2,
            peak_frames: 2,
            hits: 1,
            effectful_misses: 3,
        },
    )?;
    verify_kind(
        "write-behind",
        observation.write_behind,
        ExpectedKind {
            attempts: 3,
            admissions: 2,
            completions: 2,
            peak_frames: 1,
            hits: 0,
            effectful_misses: 2,
        },
    )
}

#[derive(Clone, Copy)]
struct ExpectedKind {
    attempts: u64,
    admissions: u64,
    completions: u64,
    peak_frames: u32,
    hits: u64,
    effectful_misses: u64,
}

fn verify_kind(
    label: &str,
    actual: BoundedResidencySpeculativeKindObservation,
    expected: ExpectedKind,
) -> Result<(), String> {
    if actual.attempts != expected.attempts
        || actual.admissions != expected.admissions
        || actual.denials != 1
        || actual.completions != expected.completions
        || actual.peak_frames != expected.peak_frames
        || actual.terminal_frames != 0
        || actual.hits != expected.hits
        || actual.effectful_misses != expected.effectful_misses
        || actual.hit_signal_requests != 0
        || actual.denial_signal_requests != 0
        || actual.effectful_signal_requests != expected.effectful_misses
        || !actual.signal_family_exact
        || !actual.foundational_basis_exact
    {
        return Err(format!(
            "Courtroom C {label} speculation did not reconcile counters, Signal, and basis: \
             {actual:?}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_schedule_is_accepted_and_each_semantic_drift_is_rejected() {
        let exact = BoundedResidencySpeculationObservation {
            prefetch: kind(5, 4, 4, 2, 1, 3),
            read_ahead: kind(3, 2, 2, 2, 1, 3),
            write_behind: kind(3, 2, 2, 1, 0, 2),
        };
        assert!(verify(exact).is_ok());

        for mutant in prefetch_mutants(exact) {
            assert!(verify(mutant).unwrap_err().contains("prefetch"));
        }
        let mut read_ahead_basis = exact;
        read_ahead_basis.read_ahead.foundational_basis_exact = false;
        assert!(verify(read_ahead_basis).unwrap_err().contains("read-ahead"));
        let mut write_behind_denial = exact;
        write_behind_denial.write_behind.denial_signal_requests = 1;
        assert!(verify(write_behind_denial)
            .unwrap_err()
            .contains("write-behind"));
    }

    fn prefetch_mutants(
        exact: BoundedResidencySpeculationObservation,
    ) -> Vec<BoundedResidencySpeculationObservation> {
        let mut mutants = Vec::with_capacity(13);
        mutate(exact, &mut mutants, |kind| kind.attempts -= 1);
        mutate(exact, &mut mutants, |kind| kind.admissions -= 1);
        mutate(exact, &mut mutants, |kind| kind.denials = 0);
        mutate(exact, &mut mutants, |kind| kind.completions -= 1);
        mutate(exact, &mut mutants, |kind| kind.peak_frames -= 1);
        mutate(exact, &mut mutants, |kind| kind.terminal_frames = 1);
        mutate(exact, &mut mutants, |kind| kind.hits = 0);
        mutate(exact, &mut mutants, |kind| kind.effectful_misses -= 1);
        mutate(exact, &mut mutants, |kind| kind.hit_signal_requests = 1);
        mutate(exact, &mut mutants, |kind| kind.denial_signal_requests = 1);
        mutate(exact, &mut mutants, |kind| {
            kind.effectful_signal_requests -= 1;
        });
        mutate(exact, &mut mutants, |kind| kind.signal_family_exact = false);
        mutate(exact, &mut mutants, |kind| {
            kind.foundational_basis_exact = false;
        });
        mutants
    }

    fn mutate(
        exact: BoundedResidencySpeculationObservation,
        mutants: &mut Vec<BoundedResidencySpeculationObservation>,
        mutation: impl FnOnce(&mut BoundedResidencySpeculativeKindObservation),
    ) {
        let mut mutant = exact;
        mutation(&mut mutant.prefetch);
        mutants.push(mutant);
    }

    fn kind(
        attempts: u64,
        admissions: u64,
        completions: u64,
        peak_frames: u32,
        hits: u64,
        effectful_misses: u64,
    ) -> BoundedResidencySpeculativeKindObservation {
        BoundedResidencySpeculativeKindObservation {
            attempts,
            admissions,
            denials: 1,
            completions,
            peak_frames,
            terminal_frames: 0,
            hits,
            effectful_misses,
            hit_signal_requests: 0,
            denial_signal_requests: 0,
            effectful_signal_requests: effectful_misses,
            signal_family_exact: true,
            foundational_basis_exact: true,
        }
    }
}
