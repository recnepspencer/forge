use forge_proof::TransitionOutcome;
use forge_store_budgets::CounterEvidenceStrength;
use forge_store_contracts::StableDigest;
use forge_store_security::StoreTenantScope;

use crate::placement::admission::test_support::{
    admit_cold_placement, admit_external_placement, admit_inline_placement,
};
use crate::test_support::{blob_scope, integrity_proof_for_scope};
use crate::{
    reject_copied_counters_as_lifecycle_receipt, reject_copied_digest_string_as_lifecycle_receipt,
    reject_imported_manifest_text_as_lifecycle_receipt,
    reject_s3_integrity_report_as_lifecycle_receipt,
    reject_terminal_projection_row_as_lifecycle_receipt, AuthenticatedFrameDigest,
    BlobAuthorityClassification, BlobChunkReachabilityRegistry, BlobGeneration,
    BlobLifecycleAdmission, BlobLifecycleDeclaration, BlobLifecycleDenial,
    BlobLifecycleReadinessAuthority, BlobLifecycleReplayInput, BlobLifecycleStoreAuthority,
    BlobObjectId, BlobReachabilityDenial, ChunkTreeRoot, LifecycleReceipt, LogicalContentDigest,
    ScopedBlobChunk, StoredChunkDigest,
};

#[test]
fn admitted_lifecycle_replay_produces_equivalent_receipts_and_exact_counters() {
    let first = execute_replay("phase1-equivalence-a");
    let second = execute_replay("phase1-equivalence-a");

    assert_eq!(first, second);
    assert_eq!(first.counters().strength(), CounterEvidenceStrength::Exact);
    assert_eq!(first.counters().declarations(), 1);
    assert_eq!(first.counters().authority_resolutions(), 1);
    assert_eq!(first.counters().lowered_plans(), 1);
    assert_eq!(first.counters().scoped_chunks(), 1);
    assert_eq!(first.counters().reachability_admissions(), 1);
    assert_eq!(first.counters().placement_admissions(), 1);
    assert_eq!(first.counters().execution_ready(), 1);
    assert_eq!(first.counters().executed_receipts(), 1);
    assert_eq!(first.counters().denials(), 0);
}

#[test]
fn copied_artifacts_have_typed_lifecycle_denials() {
    let receipt = execute_replay("phase1-denial-counter-source");
    let counters = receipt.counters();

    assert_eq!(
        reject_copied_digest_string_as_lifecycle_receipt("sha256:copied"),
        BlobLifecycleDenial::CopiedDigestStringRejected
    );
    assert_eq!(
        reject_copied_counters_as_lifecycle_receipt(counters),
        BlobLifecycleDenial::CopiedCounterSnapshotRejected { counters }
    );
    assert_eq!(
        reject_s3_integrity_report_as_lifecycle_receipt(&"s3 report"),
        BlobLifecycleDenial::S3IntegrityReportRejected
    );
    assert_eq!(
        reject_terminal_projection_row_as_lifecycle_receipt(&"projection row"),
        BlobLifecycleDenial::TerminalProjectionRowRejected
    );
    assert_eq!(
        reject_imported_manifest_text_as_lifecycle_receipt("manifest"),
        BlobLifecycleDenial::ImportedManifestTextRejected
    );
}

#[test]
fn replay_digest_mismatch_is_denied_with_exact_counter_state() {
    let ready = lifecycle_ready("phase1-mismatch");
    let mismatch = StoredChunkDigest::from_declared_digest(digest("sha256:not-the-chunk"));

    match ready
        .execute_lifecycle_replay(BlobLifecycleReplayInput::from_stored_chunk_digest(mismatch))
    {
        TransitionOutcome::Denied(BlobLifecycleDenial::ReplayStoredChunkDigestMismatch {
            counters,
        }) => {
            assert_eq!(counters.strength(), CounterEvidenceStrength::Exact);
            assert_eq!(counters.denials(), 1);
            assert_eq!(counters.executed_receipts(), 0);
        }
        outcome => panic!("expected replay denial, got {outcome:?}"),
    }
}

#[test]
fn declaration_digest_mismatch_is_denied_before_placement_or_receipt() {
    let case = "phase1-declaration-mismatch";
    let declaration = declaration_with_stored_digest(case, digest("sha256:false-declaration"));

    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    match registry.admit_lifecycle_primary_reference(&declaration, scoped_chunk(case)) {
        Err(BlobReachabilityDenial::WrongBlobAuthority { counters }) => {
            assert_eq!(counters.strength(), CounterEvidenceStrength::Exact);
            assert_eq!(counters.wrong_authority_denials(), 1);
        }
        outcome => panic!("expected reachability registry denial, got {outcome:?}"),
    }
}

#[test]
fn reachability_proof_from_another_blob_authority_is_denied() {
    let shared_chunk = scoped_chunk("phase14-shared");
    let shared_digest = shared_chunk.stored_digest().digest().clone();
    let source_declaration = declaration_with_stored_digest_and_security(
        "phase14-authority-source",
        shared_digest.clone(),
        shared_chunk.security_metadata(),
    );
    let source_reachability = reachability_proof_for_declaration(&source_declaration, shared_chunk);
    let target_declaration =
        declaration_with_stored_digest("phase14-authority-target", shared_digest);
    let store_authority = BlobLifecycleStoreAuthority::from_current_store_authority(
        current_authority("phase14-target"),
    );
    let lowering = store_authority.lowering_capability();

    assert!(matches!(
        BlobLifecycleAdmission::start(target_declaration)
            .resolve_store_authority(store_authority)
            .lower_lifecycle_plan(lowering)
            .admit_reachability(source_reachability),
        TransitionOutcome::Denied(
            BlobLifecycleDenial::DeclarationReachabilityDigestMismatch { .. }
        )
    ));
}

#[test]
fn admitted_placement_classes_execute_lifecycle_without_changing_blob_basis() {
    let inline = execute_replay_with_placement("phase16-inline", admit_inline_placement);
    let external = execute_replay_with_placement("phase16-external", admit_external_placement);
    let cold = execute_replay_with_placement("phase16-cold", admit_cold_placement);

    assert_eq!(
        inline.reachability().security_metadata(),
        inline.placement().security_metadata()
    );
    assert_eq!(
        external.reachability().security_metadata(),
        external.placement().security_metadata()
    );
    assert_eq!(
        cold.reachability().security_metadata(),
        cold.placement().security_metadata()
    );
    assert_eq!(inline.placement().counters().inline_reads(), 1);
    assert_eq!(external.placement().counters().external_reads(), 1);
    assert_eq!(cold.placement().counters().cold_fetches(), 1);
    assert_eq!(inline.counters().executed_receipts(), 1);
    assert_eq!(external.counters().executed_receipts(), 1);
    assert_eq!(cold.counters().executed_receipts(), 1);
}

#[test]
fn placement_from_another_reachability_basis_is_denied_before_receipt() {
    let shared_bytes = b"phase16-shared-placement-bytes";
    let source_chunk = scoped_chunk_with_bytes_and_tenant(
        "phase16-placement-source",
        shared_bytes,
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let target_chunk = scoped_chunk_with_bytes_and_tenant(
        "phase16-placement-target",
        shared_bytes,
        StoreTenantScope::MultiTenantPhysicalBoundary,
    );
    let shared_digest = source_chunk.stored_digest().digest().clone();
    let source_declaration = declaration_with_stored_digest_and_security(
        "phase16-placement-source",
        shared_digest.clone(),
        source_chunk.security_metadata(),
    );
    let target_declaration = declaration_with_stored_digest_and_security(
        "phase16-placement-target",
        shared_digest,
        target_chunk.security_metadata(),
    );
    let source_reachability = reachability_proof_for_declaration(&source_declaration, source_chunk);
    let target_reachability = reachability_proof_for_declaration(&target_declaration, target_chunk);
    let source_placement = admit_inline_placement(&source_reachability);
    let store_authority = BlobLifecycleStoreAuthority::from_current_store_authority(
        current_authority("phase16-placement-target"),
    );
    let lowering = store_authority.lowering_capability();

    match BlobLifecycleAdmission::start(target_declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(target_reachability)
        .success("target reachability should admit")
        .admit_placement(source_placement)
    {
        TransitionOutcome::Denied(BlobLifecycleDenial::PlacementReachabilityBasisMismatch {
            counters,
        }) => assert_eq!(counters.denials(), 1),
        outcome => panic!("expected placement basis mismatch denial, got {outcome:?}"),
    }
}

fn execute_replay(case: &str) -> LifecycleReceipt {
    let ready = lifecycle_ready(case);
    let replay = ready.admitted_replay_input();
    match ready.execute_lifecycle_replay(replay) {
        TransitionOutcome::Success(executed) => executed.into_lifecycle_receipt(),
        outcome => panic!("lifecycle replay should execute: {outcome:?}"),
    }
}

fn execute_replay_with_placement(
    case: &str,
    admit_placement: fn(&crate::BlobChunkReachabilityProofSet) -> crate::AdmittedBlobPlacement,
) -> LifecycleReceipt {
    let store_authority =
        BlobLifecycleStoreAuthority::from_current_store_authority(current_authority(case));
    let lowering = store_authority.lowering_capability();
    let declaration = declaration(case);
    let reachability = reachability_proof_for_declaration(&declaration, scoped_chunk(case));
    let placement = admit_placement(&reachability);
    let readiness = BlobLifecycleReadinessAuthority::from_admitted_placement(placement.clone());
    let ready = BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(reachability)
        .success("reachability should admit")
        .admit_placement(placement)
        .success("placement should admit")
        .ready_for_execution(readiness)
        .success("readiness should admit");
    let replay = ready.admitted_replay_input();
    ready
        .execute_lifecycle_replay(replay)
        .success("lifecycle replay should execute")
        .into_lifecycle_receipt()
}

fn lifecycle_ready(case: &str) -> crate::BlobLifecycleExecutionReady {
    let store_authority =
        BlobLifecycleStoreAuthority::from_current_store_authority(current_authority(case));
    let lowering = store_authority.lowering_capability();
    let declaration = declaration(case);
    let reachability = reachability_proof_for_declaration(&declaration, scoped_chunk(case));
    let placement = admit_inline_placement(&reachability);
    let readiness = BlobLifecycleReadinessAuthority::from_admitted_placement(placement.clone());
    BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(reachability)
        .success("reachability should admit")
        .admit_placement(placement)
        .success("placement should admit")
        .ready_for_execution(readiness)
        .success("readiness should admit")
}

fn reachability_proof_for_declaration(
    declaration: &BlobLifecycleDeclaration,
    scoped_chunk: ScopedBlobChunk,
) -> crate::BlobChunkReachabilityProofSet {
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry
        .admit_lifecycle_primary_reference(declaration, scoped_chunk)
        .expect("lifecycle reachability should admit")
}

fn scoped_chunk(case: &str) -> ScopedBlobChunk {
    scoped_chunk_with_bytes(case, chunk_digest(case).as_str().as_bytes())
}

fn scoped_chunk_with_bytes(case: &str, bytes: &[u8]) -> ScopedBlobChunk {
    scoped_chunk_with_bytes_and_tenant(case, bytes, StoreTenantScope::TenantPhysicalBoundary)
}

fn scoped_chunk_with_bytes_and_tenant(
    case: &str,
    bytes: &[u8],
    tenant_scope: StoreTenantScope,
) -> ScopedBlobChunk {
    let scope = blob_scope(case, tenant_scope);
    ScopedBlobChunk::from_integrity_proof(integrity_proof_for_scope(scope, bytes))
}

fn declaration(case: &str) -> BlobLifecycleDeclaration {
    declaration_with_stored_digest(case, scoped_chunk(case).stored_digest().digest().clone())
}

fn declaration_with_stored_digest(
    case: &str,
    stored_digest: StableDigest,
) -> BlobLifecycleDeclaration {
    declaration_with_stored_digest_and_security(
        case,
        stored_digest,
        scoped_chunk(case).security_metadata(),
    )
}

fn declaration_with_stored_digest_and_security(
    case: &str,
    stored_digest: StableDigest,
    security_metadata: crate::BlobChunkSecurityMetadataWitness,
) -> BlobLifecycleDeclaration {
    BlobLifecycleDeclaration::new(
        BlobObjectId::from_declared_digest(digest(&format!("sha256:{case}-object"))),
        BlobGeneration::published(1),
        ChunkTreeRoot::from_declared_digest(digest(&format!("sha256:{case}-root"))),
        LogicalContentDigest::from_declared_digest(digest(&format!("sha256:{case}-logical"))),
        security_metadata,
        StoredChunkDigest::from_declared_digest(stored_digest),
        AuthenticatedFrameDigest::from_declared_digest(digest(&format!("sha256:{case}-frame"))),
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    )
}

fn chunk_digest(case: &str) -> StableDigest {
    digest(&format!("sha256:{case}-chunk"))
}

fn digest(raw: &str) -> StableDigest {
    StableDigest::new(raw).expect("stable digest")
}

fn current_authority(case: &str) -> forge_store_authority::StoreCurrentAuthorityWitness {
    use forge_foundational::{
        aspects, AspectContract, AspectValue, InternedString, ScalarAspectType,
    };
    use forge_store_aspect_native::{
        StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
        StorePhysicalBoundaryWitness,
    };
    use forge_store_authority::require_current_store_authority;
    use forge_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };

    let key = aspects()
        .vocabulary()
        .key(&format!("s7.lifecycle.{case}"))
        .expect("aspect key");
    let contract: AspectContract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let value = aspects()
        .validate()
        .against(&contract)
        .value(AspectValue::String(InternedString::from(case)))
        .success("aspect value should validate");
    let admitted_state = aspects()
        .authoritative_state()
        .admit([value])
        .success("aspect state should admit");
    let physical = StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary");
    let boundary = StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical),
    )
    .expect("boundary fact");
    require_current_store_authority(boundary)
}

trait TestTransitionSuccess<S> {
    fn success(self, message: &str) -> S;
}

impl<S, D, De, St, R, F> TestTransitionSuccess<S> for TransitionOutcome<S, D, De, St, R, F>
where
    S: core::fmt::Debug,
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    fn success(self, message: &str) -> S {
        match self {
            TransitionOutcome::Success(value) => value,
            outcome => panic!("{message}: {outcome:?}"),
        }
    }
}
