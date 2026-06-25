use crate::validator_invariant_catalog::{
    WorthTopologyRelationalInvariantCatalogDenialKind,
    WorthTopologyRelationalInvariantOldPackResidueStatus,
    WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission,
    WorthTopologyRelationalInvariantRejectedAuthorityKind,
};

use super::execution_inputs::relational_invariant_closeout;

#[test]
fn old_milestone_one_pack_is_source_intake_not_ordinary_path() {
    let closeout = relational_invariant_closeout();
    let residue = closeout.old_pack_residue();

    assert_eq!(residue.ordinary_path_count(), 0);
    assert!(residue.source_pack_registration_count() > 0);
    assert!(residue.rows().iter().any(|row| {
        row.status()
            == WorthTopologyRelationalInvariantOldPackResidueStatus::CertificationOnlySourceIntake
            && row.registration_count() == residue.source_pack_registration_count()
            && !row.owner().is_empty()
            && !row.blocker().is_empty()
            && !row.removal_trigger().is_empty()
    }));
    assert!(residue.rows().iter().all(|row| {
        !row.owner().is_empty() && !row.blocker().is_empty() && !row.removal_trigger().is_empty()
    }));
    assert_eq!(
        closeout.counters().old_pack_ordinary_path_count(),
        residue.ordinary_path_count()
    );
}

#[test]
fn static_and_manual_packs_are_rejected_as_ordinary_authority() {
    for kind in [
        WorthTopologyRelationalInvariantRejectedAuthorityKind::StaticInvariantPack,
        WorthTopologyRelationalInvariantRejectedAuthorityKind::ManualGraphCompositionInvariantPack,
        WorthTopologyRelationalInvariantRejectedAuthorityKind::ExplicitRelationalRuntimeAuthority,
    ] {
        let denial =
            WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission::reject_non_query_authority(
                kind,
                format!("attempted-authority:{}", kind.as_str()),
            );

        assert_eq!(
            denial.kind(),
            WorthTopologyRelationalInvariantCatalogDenialKind::RejectedNonQueryAuthority
        );
        assert!(denial.detail().contains(kind.as_str()));
        assert!(denial
            .detail()
            .contains("Query-selected graph-scoped registrations"));
    }
}
