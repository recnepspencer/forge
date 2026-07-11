use crate::courtroom::source_tree::repository_root;
use crate::{
    certify_scanned_store_canonical_basis_source_inventory,
    certify_store_canonical_basis_source_inventory, certify_store_canonical_basis_source_rows,
    courtroom::foundational::canonical_basis_source_registry::STORE_CANONICAL_BASIS_FAMILY_REGISTRY,
    courtroom::foundational::canonical_basis_source_scan::scan_store_canonical_basis_family_surfaces,
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
    assert_eq!(inventory.len(), STORE_CANONICAL_BASIS_FAMILY_REGISTRY.len());

    for row in inventory {
        let family = row.family().expect("registered rows carry owner family");
        let owner = canonical_basis_source_owner_for_family(family)
            .expect("family must have a source owner");
        assert!(!owner.owner_crate().is_empty());
        assert!(!owner.classifying_subsystem().is_empty());
    }
}

#[test]
fn source_map_covers_scanned_store_workspace_family_surfaces() {
    let workspace_root = repository_root();
    let scope_roots = [
        "workspaces/forge-store/crates/forge-store-aspect-native/src",
        "workspaces/forge-store/crates/forge-store-certification/src",
        "workspaces/forge-store/crates/forge-store-readiness/src",
        "workspaces/forge-store/crates/forge-store-physical-format/src",
        "workspaces/forge-store/crates/forge-store-physical-integrity/src",
        "workspaces/forge-store/crates/forge-store-recovery-physics/src",
        "workspaces/forge-store/crates/forge-store-wal/src",
    ];
    let scanned = scan_store_canonical_basis_family_surfaces(workspace_root, &scope_roots);

    assert!(scanned.iter().any(|family| {
        family.family_name() == "DerivedIndexAuthorityEvidence"
            && family.source_path()
                == "workspaces/forge-store/crates/forge-store-physical-integrity/src/index_pages/index_page_integrity_request.rs"
    }));
    assert!(scanned.iter().any(|family| {
        family.family_name() == "S3ExecutedBoundaryDenialEvidence"
            && family.source_path()
                == "workspaces/forge-store/crates/forge-store-certification/src/courtroom/physical_integrity/physical_integrity_closeout_proof.rs"
    }));
    assert!(scanned.iter().any(|family| {
        family.family_name() == "PhysicalRecoverySource"
            && family.source_path()
                == "workspaces/forge-store/crates/forge-store-recovery-physics/src/source_precedence/physical_source.rs"
    }));
    assert!(scanned.iter().any(|family| {
        family.family_name() == "RecoveryBlockingIntegritySource"
            && family.source_path()
                == "workspaces/forge-store/crates/forge-store-recovery-physics/src/recovery_blocking_integrity.rs"
    }));
    assert!(scanned.iter().any(|family| {
        family.family_name() == "RecoveryPhysicsIntegrityInput"
            && family.source_path()
                == "workspaces/forge-store/crates/forge-store-recovery-physics/src/integrity_input.rs"
    }));

    certify_scanned_store_canonical_basis_source_inventory(workspace_root, &scope_roots)
        .expect("source owner registry must cover every scanned Store family surface");
}

#[test]
fn unclassified_store_evidence_family_fails_with_classifying_subsystem() {
    let denial = certify_store_canonical_basis_source_rows(&[
        StoreCanonicalBasisInventoryRow::unclassified(
            "new recovery receipt family",
            "workspaces/forge-store/crates/forge-store-recovery-physics/src/new_receipt.rs",
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
