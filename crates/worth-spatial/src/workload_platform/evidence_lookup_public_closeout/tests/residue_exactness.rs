use crate::workload_platform::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout,
    current_evidence_lookup_public_closeout_residue_manifest,
    EvidenceLookupPublicCloseoutDisposition,
};

#[test]
fn exported_residue_manifest_matches_live_public_closeout_residue() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let manifest = current_evidence_lookup_public_closeout_residue_manifest();
    let live_residue_count = closeout
        .family_stage_rows()
        .iter()
        .filter(|row| {
            matches!(
                row.disposition(),
                EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. }
            )
        })
        .count();

    assert_eq!(manifest.len(), live_residue_count);
    assert_eq!(manifest.len(), 0);
    assert!(closeout.family_stage_rows().iter().all(|row| {
        matches!(
            row.disposition(),
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
        )
    }));
}
