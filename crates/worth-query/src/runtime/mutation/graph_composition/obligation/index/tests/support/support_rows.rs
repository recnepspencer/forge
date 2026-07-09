use std::collections::BTreeSet;

use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationIndexSupportStatus,
    WorthQueryGraphObligationSupportLane,
};

#[test]
fn index_support_rows_cover_every_obligation_kind_for_assembly_selection() {
    let index = WorthQueryGraphObligationIndex::empty();
    let rows = index.support_rows();

    assert_eq!(rows.len(), 6);
    assert!(rows
        .iter()
        .all(|row| row.status() == WorthQueryGraphObligationIndexSupportStatus::Verified));
    assert!(rows
        .iter()
        .all(|row| row.lane() == WorthQueryGraphObligationSupportLane::AssemblyIndexSelection));
    assert!(rows
        .iter()
        .all(|row| row.lane_label() == "assembly-index-selection"));
    assert_eq!(
        rows.iter()
            .map(|row| row.obligation_kind().as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "advisory-obligation",
            "blocking-invariant",
            "capability-gap-screen",
            "operating-context-gate",
            "preflight-sequencing-obligation",
            "schema-contract-validator",
        ])
    );
}
