use crate::authority::commit::preparation::diagnostics::observations::ValidationDiagnosticObservation;

pub(crate) fn sort_diagnostic_observations(
    observations: &mut [ValidationDiagnosticObservation],
) {
    observations.sort_by(|left, right| {
        left.packet_index
            .cmp(&right.packet_index)
            .then_with(|| left.result_identity.cmp(&right.result_identity))
    });
}
