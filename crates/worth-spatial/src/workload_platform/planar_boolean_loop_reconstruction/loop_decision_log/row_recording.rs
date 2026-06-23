use std::collections::BTreeSet;

use super::counters::PlanarBooleanLoopDecisionLogCounters;
use super::denial::{
    PlanarBooleanLoopDecisionLogDenial, PlanarBooleanLoopDecisionLogDenialKind as Kind,
};
use super::input::PlanarBooleanLoopDecisionLogInput;
use super::row::PlanarBooleanLoopDecisionRow;
use super::row_recording_core::record_core_rows;
use super::row_recording_identity::record_identity_rows;

pub(crate) fn record_rows(
    input: PlanarBooleanLoopDecisionLogInput<'_>,
    counters: &mut PlanarBooleanLoopDecisionLogCounters,
) -> Result<Vec<PlanarBooleanLoopDecisionRow>, PlanarBooleanLoopDecisionLogDenial> {
    let mut rows = Vec::new();
    let mut seen_decision_identities = BTreeSet::new();
    record_core_rows(input, &mut rows, &mut seen_decision_identities, counters)?;
    record_identity_rows(input, &mut rows, &mut seen_decision_identities, counters)?;
    Ok(rows)
}

pub(super) fn push_row(
    rows: &mut Vec<PlanarBooleanLoopDecisionRow>,
    seen_decision_identities: &mut BTreeSet<String>,
    counters: &mut PlanarBooleanLoopDecisionLogCounters,
    row: PlanarBooleanLoopDecisionRow,
) -> Result<(), PlanarBooleanLoopDecisionLogDenial> {
    if !seen_decision_identities.insert(row.decision_identity().to_string()) {
        counters.denied_duplicate_decision_identity();
        return Err(PlanarBooleanLoopDecisionLogDenial::new(
            Kind::DuplicateDecisionIdentity,
            row.decision_identity(),
            *counters,
            "loop decision-log identities must be unique",
        ));
    }
    counters.emitted_decision_row();
    rows.push(row);
    Ok(())
}
