use super::execution_inputs::relational_invariant_closeout;

#[test]
fn phase_six_seed_preserves_catalog_query_residue_and_no_execution_boundary() {
    let closeout = relational_invariant_closeout();
    let seed = closeout.phase_six_seed();

    assert_eq!(seed.catalog_digest(), closeout.catalog_digest());
    assert_eq!(seed.selected_plan_digest(), closeout.selected_plan_digest());
    assert_eq!(
        seed.query_registration_projection_digest(),
        closeout.query_registration_artifact().projection_digest()
    );
    assert_eq!(
        seed.query_registration_bundle_digest(),
        closeout.query_registration_bundle().bundle_digest()
    );
    assert_eq!(
        seed.ordinary_authority_admission_digest(),
        closeout.ordinary_authority_admission().admission_digest()
    );
    assert_eq!(
        seed.old_pack_residue_digest(),
        closeout.old_pack_residue().report_digest()
    );
    assert_eq!(
        seed.source_firewall_digest(),
        closeout.source_firewall().report_digest()
    );
    assert_eq!(
        seed.selected_invariant_family_count(),
        closeout.selected_invariant_family_rows().len()
    );
    assert_eq!(
        seed.selected_validator_family_count(),
        closeout.selected_validator_family_rows().len()
    );
    assert_eq!(
        seed.selected_invariant_family_row_digests(),
        closeout
            .selected_invariant_family_rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.selected_validator_family_row_digests(),
        closeout
            .selected_validator_family_rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.query_graph_obligation_registration_digests(),
        closeout
            .query_registration_bundle()
            .graph_obligation_registrations()
            .iter()
            .map(|registration| registration.registration_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(seed.execution_receipt_count(), 0);
}
