use crate::workload_platform::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;

#[test]
fn declare_once_lookup_proof_applies_after_closeout() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");

    let event_rows = closeout
        .family_stage_rows()
        .iter()
        .filter(|row| row.family_identity() == "spatial-touch.boolean.event-ledger-evidence.v1")
        .collect::<Vec<_>>();
    let projection_rows = closeout
        .family_stage_rows()
        .iter()
        .filter(|row| {
            row.family_identity() == "spatial-touch.boolean.projection-consumption-evidence.v1"
        })
        .collect::<Vec<_>>();

    assert_eq!(event_rows.len(), 2);
    assert_eq!(projection_rows.len(), 2);
    assert!(event_rows.iter().all(|row| !row.row_digest().is_empty()));
    assert!(event_rows
        .windows(2)
        .all(|pair| pair[0].family_declaration_digest() == pair[1].family_declaration_digest()));
    assert!(projection_rows
        .iter()
        .all(|row| !row.row_digest().is_empty()));
    assert!(projection_rows.windows(2).all(|pair| {
        pair[0].family_declaration_digest() == pair[1].family_declaration_digest()
            && pair[0].stage_receipt_family_identity() == pair[1].stage_receipt_family_identity()
    }));
}
