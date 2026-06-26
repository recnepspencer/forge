use crate::{
    certify_store_canonical_basis_source_inventory, certify_store_canonical_basis_source_rows,
    current_store_canonical_basis_inventory, StoreCanonicalBasisInventoryDenial,
    StoreCanonicalBasisInventoryRow,
};
use forge_store_aspect_native::{
    canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
    certify_canonical_basis_source, StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole,
    StoreCanonicalBasisLane, StoreCanonicalBasisSourceDenial, StoreCanonicalBasisSourceKind,
};

#[test]
fn current_store_evidence_families_have_canonical_basis_source_owners() {
    certify_store_canonical_basis_source_inventory()
        .expect("all current Store families must have source owners");

    let inventory = current_store_canonical_basis_inventory();
    assert_eq!(inventory.len(), StoreCanonicalBasisFamily::ALL.len());

    for family in StoreCanonicalBasisFamily::ALL {
        let owner = canonical_basis_source_owner_for_family(family)
            .expect("family must have a source owner");
        assert!(!owner.owner_crate().is_empty());
        assert!(!owner.classifying_subsystem().is_empty());
    }
}

#[test]
fn unclassified_store_evidence_family_fails_with_classifying_subsystem() {
    let denial = certify_store_canonical_basis_source_rows(&[
        StoreCanonicalBasisInventoryRow::unclassified(
            "new recovery receipt family",
            "forge-store-recovery-physics",
        ),
    ])
    .expect_err("unclassified Store evidence families must fail the gate");

    assert_eq!(
        denial,
        StoreCanonicalBasisInventoryDenial::UnclassifiedEvidenceFamily {
            family_name: "new recovery receipt family",
            classifying_subsystem: "forge-store-recovery-physics",
        }
    );
}

#[test]
fn terminal_and_text_fields_cannot_supply_canonical_basis() {
    for field_role in [
        StoreCanonicalBasisFieldRole::TerminalProjection,
        StoreCanonicalBasisFieldRole::OperatorDisplay,
        StoreCanonicalBasisFieldRole::DocumentChecksum,
        StoreCanonicalBasisFieldRole::CompatibilityText,
        StoreCanonicalBasisFieldRole::DigestText,
        StoreCanonicalBasisFieldRole::RawJsonPayload,
    ] {
        assert_eq!(
            certify_canonical_basis_field_role(field_role),
            Err(StoreCanonicalBasisSourceDenial::ForbiddenFieldRole { field_role })
        );
    }
}

#[test]
fn native_source_kinds_are_distinct_across_similar_families() {
    assert!(certify_canonical_basis_source(
        StoreCanonicalBasisFamily::PhysicalSourceManifest,
        StoreCanonicalBasisSourceKind::StoreSourceManifest,
    )
    .is_ok());
    assert!(certify_canonical_basis_source(
        StoreCanonicalBasisFamily::PhysicalPageHeader,
        StoreCanonicalBasisSourceKind::StorePageHeader,
    )
    .is_ok());
    assert!(certify_canonical_basis_source(
        StoreCanonicalBasisFamily::RecoveryWalReplayReceipt,
        StoreCanonicalBasisSourceKind::StoreRecoveryReceipt,
    )
    .is_ok());

    assert_eq!(
        certify_canonical_basis_source(
            StoreCanonicalBasisFamily::PhysicalPageHeader,
            StoreCanonicalBasisSourceKind::StoreSourceManifest,
        ),
        Err(StoreCanonicalBasisSourceDenial::WrongNativeSourceKind {
            family: StoreCanonicalBasisFamily::PhysicalPageHeader,
            source: StoreCanonicalBasisSourceKind::StoreSourceManifest,
        })
    );
}

#[test]
fn source_roles_keep_handoffs_performance_and_recovery_separate() {
    let handoff = canonical_basis_source_owner_for_family(
        StoreCanonicalBasisFamily::S3IntegrityCloseoutHandoff,
    )
    .unwrap();
    let performance = canonical_basis_source_owner_for_family(
        StoreCanonicalBasisFamily::PerformanceReceiptEvidence,
    )
    .unwrap();
    let recovery = canonical_basis_source_owner_for_family(
        StoreCanonicalBasisFamily::RecoveryCheckpointValidityReceipt,
    )
    .unwrap();

    assert_eq!(
        handoff.foundational_lane(),
        StoreCanonicalBasisLane::Handoff
    );
    assert_eq!(
        performance.foundational_lane(),
        StoreCanonicalBasisLane::PerformanceEvidence
    );
    assert_eq!(
        recovery.foundational_lane(),
        StoreCanonicalBasisLane::Recovery
    );
}
