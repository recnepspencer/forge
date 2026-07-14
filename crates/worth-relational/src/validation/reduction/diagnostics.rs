use crate::authority::commit::preparation::diagnostics::observations::ValidationDiagnosticObservation;

pub(crate) fn assert_canonical_diagnostic_observations(
    observations: &[ValidationDiagnosticObservation],
) {
    debug_assert!(
        observations
            .windows(2)
            .all(|window| window[0].canonical_key() <= window[1].canonical_key()),
        "validation diagnostic observations must already be in canonical order"
    );
}
