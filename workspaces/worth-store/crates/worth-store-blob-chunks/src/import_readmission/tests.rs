use super::{BlobImportPlacementDisposition, BlobImportPlacementSource};
use worth_store_security::{StoreKeyVersionPosture, StoreTrustBoundaryCrossing};

use super::test_support::{collect_current_chunks, import_lane, readmission_trigger};
use super::{
    bridge_canonical_export_trust_boundary, parse_import_declaration_json,
    reject_copied_export_row_as_blob_import,
    reject_placement_only_evidence_as_imported_blob_witness, BlobImportChunkDeclaration,
    BlobImportDeclaration, BlobImportReadmissionAuthority, BlobImportReadmissionDenial,
    BoundaryBridgedCanonicalExportArtifact,
};
use crate::test_support::current_authority;
use crate::BlobChunkByteWindow;

#[test]
fn export_followed_by_readmitted_import_reconstructs_same_identity_digests_scope_and_counters() {
    let lane = import_lane("phase20.import", b"aaaabbbbcccc", 12);
    let import_authority = BlobImportReadmissionAuthority::from_current_store_authority(
        current_authority("phase20.import", "import"),
    );
    let bridged = bridge_canonical_export_trust_boundary(&lane.bundle);
    let current_chunks = collect_current_chunks(&import_authority, &lane);
    let declaration = bridged.clone().into_declaration();
    let trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        declaration.chunk_scope(),
        "phase20.import",
    );
    let readmitted = import_authority
        .readmit_import_declaration_after_boundary(&bridged, trigger, &current_chunks)
        .expect("readmission should admit");
    let placement_plan = readmitted
        .plan_placement_admission()
        .expect("placement plan should admit");
    let witness = readmitted
        .admit_imported_blob(&placement_plan)
        .expect("witness should admit");

    assert_eq!(witness.object_id(), lane.bundle.object_id());
    assert_eq!(witness.generation(), lane.bundle.generation());
    assert_eq!(witness.chunk_tree_root(), lane.bundle.chunk_tree_root());
    assert_eq!(
        witness.logical_content_digest(),
        lane.bundle.digest_evidence().logical_content_digest()
    );
    assert_eq!(witness.security_metadata(), lane.bundle.security_metadata());
    assert_eq!(
        witness.reachable_chunks(),
        lane.reachability.reachable_chunks()
    );
    assert_eq!(witness.stored_digest(), lane.reachability.stored_digest());
    assert_eq!(
        placement_plan.disposition(),
        BlobImportPlacementDisposition::AlreadyPresentLocally
    );
    assert_eq!(
        placement_plan.source(),
        BlobImportPlacementSource::InlineInBundle
    );
    assert_eq!(witness.counters().imported_declarations(), 1);
    assert_eq!(witness.counters().readmitted_chunks(), 1);
    assert_eq!(witness.counters().witness_constructions(), 1);
}

#[test]
fn imported_json_and_hostile_boundary_cases_deny_before_witness_construction() {
    let json = parse_import_declaration_json("{\"portable\":true}");
    assert!(matches!(
        json,
        Err(BlobImportReadmissionDenial::ImportedJsonRejected { .. })
    ));
    let json_counters = match json {
        Err(denial) => *denial.counters(),
        Ok(_) => panic!("json should deny"),
    };
    assert_eq!(json_counters.imported_declarations(), 1);
    assert_eq!(json_counters.terminal_projection_denials(), 1);

    let lane = import_lane("phase20.import.denial", b"aaaabbbbcccc", 12);
    let import_authority = BlobImportReadmissionAuthority::from_current_store_authority(
        current_authority("phase20.import.denial", "import"),
    );
    let bridged = bridge_canonical_export_trust_boundary(&lane.bundle);
    let current_chunks = collect_current_chunks(&import_authority, &lane);

    let stale_decl = bridged.clone().into_declaration().with_chunk_scope(
        worth_store_security::StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
            current_authority("phase20.import.denial", "import").physical_witness(),
            worth_store_security::StoreKeyScope::BlobChunkEnvelope,
            StoreKeyVersionPosture::Stale,
            worth_store_security::StoreTenantScope::TenantPhysicalBoundary,
            Some(
                worth_store_security::StoreAuthenticityRequirement::required(
                    worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
                ),
            ),
            Some(worth_store_security::StoreCustodyPosture::ImportedUnreadmitted),
        ),
    );
    let stale_artifact =
        BoundaryBridgedCanonicalExportArtifact::from_declaration(stale_decl.clone());
    let stale_trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::KeyScopeGenerationChanged,
        stale_decl.chunk_scope(),
        "phase20.import.denial",
    );
    let stale = import_authority.readmit_import_declaration_after_boundary(
        &stale_artifact,
        stale_trigger,
        &current_chunks,
    );
    let stale_counters = match stale {
        Err(BlobImportReadmissionDenial::StaleKeyGeneration { counters }) => counters,
        other => panic!("expected stale denial, got {other:?}"),
    };
    assert_eq!(stale_counters.imported_declarations(), 1);
    assert_eq!(stale_counters.stale_scope_denials(), 1);

    let tenant_decl = bridged.clone().into_declaration().with_chunk_scope(
        worth_store_security::StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
            current_authority("phase20.import.denial", "import").physical_witness(),
            worth_store_security::StoreKeyScope::BlobChunkEnvelope,
            StoreKeyVersionPosture::Current,
            worth_store_security::StoreTenantScope::MultiTenantPhysicalBoundary,
            Some(
                worth_store_security::StoreAuthenticityRequirement::required(
                    worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
                ),
            ),
            Some(worth_store_security::StoreCustodyPosture::ImportedUnreadmitted),
        ),
    );
    let tenant_artifact =
        BoundaryBridgedCanonicalExportArtifact::from_declaration(tenant_decl.clone());
    let tenant_trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged,
        tenant_decl.chunk_scope(),
        "phase20.import.denial",
    );
    let tenant = import_authority.readmit_import_declaration_after_boundary(
        &tenant_artifact,
        tenant_trigger,
        &current_chunks,
    );
    let tenant_counters = match tenant {
        Err(BlobImportReadmissionDenial::WrongTenantAuthority { counters }) => counters,
        other => panic!("expected tenant denial, got {other:?}"),
    };
    assert_eq!(tenant_counters.imported_declarations(), 1);
    assert_eq!(tenant_counters.stale_scope_denials(), 1);

    let copied_decl = bridged.clone().into_declaration();
    let mut copied_rows = copied_decl.chunk_rows().to_vec();
    copied_rows.push(copied_rows[0].clone());
    let copied_artifact = BoundaryBridgedCanonicalExportArtifact::from_declaration(
        copied_decl.with_chunk_rows(copied_rows),
    );
    let copied = import_authority.readmit_import_declaration_after_boundary(
        &copied_artifact,
        readmission_trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            bridged.clone().into_declaration().chunk_scope(),
            "phase20.import.denial",
        ),
        &current_chunks,
    );
    let copied_counters = match copied {
        Err(BlobImportReadmissionDenial::CopiedExportRowRejected { counters }) => counters,
        other => panic!("expected copied-row denial, got {other:?}"),
    };
    assert_eq!(copied_counters.imported_declarations(), 1);
    assert_eq!(copied_counters.copied_row_denials(), 1);

    let custody_decl = bridged.clone().into_declaration();
    let custody_trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        custody_decl.chunk_scope(),
        "phase20.import.denial",
    );
    let drifted = bridged
        .clone()
        .into_declaration()
        .with_export_custody_scope(
        worth_store_security::StoreSecurityScopeIdentity::from_physical_security_scope(
            current_authority("phase20.import.denial", "import").physical_witness(),
            worth_store_security::StoreKeyScope::BackupExportEnvelope,
            StoreKeyVersionPosture::Current,
            worth_store_security::StoreTenantScope::MultiTenantPhysicalBoundary,
            worth_store_security::StoreAuthenticityRequirement::required(
                worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
            ),
            worth_store_security::StoreCustodyPosture::ExportPrepared,
        ),
    );
    let drifted_artifact = BoundaryBridgedCanonicalExportArtifact::from_declaration(drifted);
    let custody = import_authority.readmit_import_declaration_after_boundary(
        &drifted_artifact,
        custody_trigger,
        &current_chunks,
    );
    let custody_counters = match custody {
        Err(BlobImportReadmissionDenial::CustodyDomainMismatch { counters }) => counters,
        other => panic!("expected custody denial, got {other:?}"),
    };
    assert_eq!(custody_counters.imported_declarations(), 1);
    assert_eq!(custody_counters.stale_scope_denials(), 1);

    let base_decl = bridged.clone().into_declaration();
    let external_artifact =
        BoundaryBridgedCanonicalExportArtifact::from_declaration(BlobImportDeclaration::portable(
            base_decl.object_id().clone(),
            base_decl.generation(),
            base_decl.chunk_tree_root().clone(),
            base_decl.logical_content_digest().clone(),
            base_decl.chunk_scope(),
            crate::BlobImportTransferDeclaration::new(
                BlobImportPlacementSource::ExternalByReference,
                base_decl.export_custody_scope(),
                base_decl
                    .chunk_rows()
                    .iter()
                    .map(|row| {
                        BlobImportChunkDeclaration::portable(
                            row.ordinal(),
                            row.chunk_identity(),
                            row.stored_digest(),
                            row.checksum_digest(),
                            row.bytes(),
                        )
                    })
                    .collect(),
            ),
        ));
    let deduped = import_authority
        .readmit_import_declaration_after_boundary(
            &external_artifact,
            readmission_trigger(
                StoreTrustBoundaryCrossing::OfflineExportImport,
                bridged.clone().into_declaration().chunk_scope(),
                "phase20.import.denial",
            ),
            &current_chunks,
        )
        .expect("external-reference import should still readmit");
    let deduped_plan = deduped
        .plan_placement_admission()
        .expect("placement planning should see explicit external reference");
    assert_eq!(
        deduped_plan.disposition(),
        BlobImportPlacementDisposition::DedupedLocally
    );
    assert_eq!(
        deduped_plan.source(),
        BlobImportPlacementSource::ExternalByReference
    );

    let missing = import_authority
        .readmit_import_declaration_after_boundary(
            &external_artifact,
            readmission_trigger(
                StoreTrustBoundaryCrossing::OfflineExportImport,
                bridged.clone().into_declaration().chunk_scope(),
                "phase20.import.denial",
            ),
            &current_chunks[..0],
        )
        .expect("external-reference readmission should survive with no local chunks");
    let requires_fetch = missing
        .plan_placement_admission()
        .expect("placement planning should see missing external chunks");
    assert_eq!(
        requires_fetch.disposition(),
        BlobImportPlacementDisposition::RequiresFetch
    );
    assert_eq!(
        requires_fetch.source(),
        BlobImportPlacementSource::ExternalByReference
    );
    let missing_witness = missing.admit_imported_blob(&requires_fetch);
    let missing_counters = match missing_witness {
        Err(BlobImportReadmissionDenial::MissingChunk { counters }) => counters,
        other => panic!("expected missing-chunk denial, got {other:?}"),
    };
    assert_eq!(missing_counters.imported_declarations(), 1);
    assert_eq!(missing_counters.readmitted_chunks(), 0);
    assert_eq!(missing_counters.missing_chunk_denials(), 1);

    let placement_only = reject_placement_only_evidence_as_imported_blob_witness(&lane.placement);
    assert!(matches!(
        placement_only,
        BlobImportReadmissionDenial::PlacementOnlyEvidenceRejected { .. }
    ));
    assert_eq!(placement_only.counters().placement_only_denials(), 1);

    let copied_row = reject_copied_export_row_as_blob_import("row");
    assert!(matches!(
        copied_row,
        BlobImportReadmissionDenial::CopiedExportRowRejected { .. }
    ));
}

#[test]
fn current_chunk_evidence_mismatch_reports_exact_pre_readmission_counters() {
    let lane = import_lane("phase20.import.collector-denial", b"aaaabbbbcccc", 12);
    let import_authority = BlobImportReadmissionAuthority::from_current_store_authority(
        current_authority("phase20.import.collector-denial", "import"),
    );
    let leaf = &lane.ordered_leaves[0];
    let bad_bytes =
        BlobChunkByteWindow::borrowed(leaf.byte_range().start(), b"wrongchunk").expect("window");

    let mismatch = import_authority.collect_current_chunk_evidence(leaf, bad_bytes);
    let counters = match mismatch {
        Err(BlobImportReadmissionDenial::ChunkEvidenceMismatch { counters }) => counters,
        other => panic!("expected chunk-evidence mismatch, got {other:?}"),
    };
    assert_eq!(counters.imported_declarations(), 0);
    assert_eq!(counters.readmitted_chunks(), 0);
    assert_eq!(counters.stale_scope_denials(), 0);
    assert_eq!(counters.missing_chunk_denials(), 0);
    assert_eq!(counters.copied_row_denials(), 0);
    assert_eq!(counters.terminal_projection_denials(), 0);
    assert_eq!(counters.placement_only_denials(), 0);
    assert_eq!(counters.witness_constructions(), 0);
}
