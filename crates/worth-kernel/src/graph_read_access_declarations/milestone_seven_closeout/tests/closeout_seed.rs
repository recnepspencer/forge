use super::common::{production_closeout, production_phase_seven_seed};

#[test]
fn milestone_seven_closeout_exports_milestone_eight_seed() {
    let closeout = production_closeout();
    let seed = closeout.milestone_eight_seed();

    assert_eq!(
        seed.milestone_seven_closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        seed.declaration_catalog_digest(),
        closeout.declaration_catalog_digest()
    );
    assert_eq!(
        seed.read_family_identities(),
        closeout.read_family_identities()
    );
    assert_eq!(
        seed.requirement_row_evidence(),
        closeout.requirement_row_evidence()
    );
    assert_eq!(
        seed.deletion_firewall_digest(),
        closeout.deletion_firewall_digest()
    );
    assert!(!seed.claims_graph_read_execution());
    assert!(!seed.claims_access_plan_consumption());
}

#[test]
fn closeout_digest_is_not_a_forwarded_component_digest() {
    let closeout = production_closeout();

    assert_ne!(
        closeout.closeout_digest(),
        closeout.declaration_catalog_digest()
    );
    assert_ne!(
        closeout.closeout_digest(),
        closeout.milestone_eight_seed().deletion_firewall_digest()
    );
    assert!(closeout
        .read_family_identities()
        .iter()
        .all(|identity| closeout.closeout_digest() != identity.identity_digest()));
    assert!(closeout
        .requirement_row_evidence()
        .iter()
        .all(|row| closeout.closeout_digest() != row.requirement_row_digest()));
}

#[test]
fn milestone_eight_seed_preserves_structured_identity_rows() {
    let phase_seven_seed = production_phase_seven_seed();
    let closeout = production_closeout();
    let seed = closeout.milestone_eight_seed();

    assert_eq!(
        seed.read_family_identities().len(),
        phase_seven_seed.posture_records().len()
    );
    assert_eq!(
        seed.requirement_row_evidence().len(),
        phase_seven_seed.posture_records().len()
    );

    for (identity, posture_record) in seed
        .read_family_identities()
        .iter()
        .zip(phase_seven_seed.posture_records())
    {
        assert_eq!(
            identity.source_catalog_record_digest(),
            posture_record.source_catalog_record_digest()
        );
        assert_eq!(
            identity.query_family_name(),
            posture_record.query_family_name()
        );
        assert_eq!(
            identity.query_family_digest_seed(),
            posture_record.query_family_digest_seed()
        );
        assert_eq!(
            identity.touched_authority_input(),
            posture_record.touched_authority_input()
        );
        assert_eq!(
            identity.read_family_target(),
            posture_record.read_family_target()
        );
        assert!(!identity.identity_digest().is_empty());
    }
    for (row, posture_record) in seed
        .requirement_row_evidence()
        .iter()
        .zip(phase_seven_seed.posture_records())
    {
        assert_eq!(
            row.source_requirement_record_digest(),
            posture_record.source_requirement_record_digest()
        );
        assert_eq!(
            row.source_catalog_record_digest(),
            posture_record.source_catalog_record_digest()
        );
        assert_eq!(
            row.query_family_digest_seed(),
            posture_record.query_family_digest_seed()
        );
        assert!(!row.requirement_row_digest().is_empty());
    }
}
