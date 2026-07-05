use forge_proof::TransitionOutcome;
use forge_store_budgets::CounterEvidenceStrength;
use forge_store_contracts::StableDigest;
use forge_store_readiness::{
    admit_s6_s7_placement_handoff, S6ClosedS7PlacementAdmissionSeed,
    S6S7PlacementAdmissionAuthority,
};
use forge_store_security::StoreTenantScope;

use crate::blob_chunk_test_support::{blob_scope, integrity_proof_for_scope};
use crate::{
    reject_copied_counters_as_lifecycle_receipt, reject_copied_digest_string_as_lifecycle_receipt,
    reject_imported_manifest_text_as_lifecycle_receipt,
    reject_s3_integrity_report_as_lifecycle_receipt,
    reject_terminal_projection_row_as_lifecycle_receipt, AuthenticatedFrameDigest,
    BlobAuthorityClassification, BlobGeneration, BlobLifecycleAdmission, BlobLifecycleDeclaration,
    BlobLifecycleDenial, BlobLifecycleReadinessAuthority, BlobLifecycleReplayInput,
    BlobLifecycleStoreAuthority, BlobObjectId, ChunkTreeRoot, LifecycleReceipt,
    LogicalContentDigest, ScopedBlobChunk, StoredChunkDigest,
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
    let store_authority =
        BlobLifecycleStoreAuthority::from_current_store_authority(current_authority(case));
    let lowering = store_authority.lowering_capability();
    let declaration = declaration_with_stored_digest(case, digest("sha256:false-declaration"));

    match BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(scoped_chunk(case))
    {
        TransitionOutcome::Denied(BlobLifecycleDenial::DeclarationReachabilityDigestMismatch {
            counters,
        }) => {
            assert_eq!(counters.strength(), CounterEvidenceStrength::Exact);
            assert_eq!(counters.denials(), 1);
            assert_eq!(counters.placement_admissions(), 0);
            assert_eq!(counters.executed_receipts(), 0);
        }
        outcome => panic!("expected declaration binding denial, got {outcome:?}"),
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

fn lifecycle_ready(case: &str) -> crate::BlobLifecycleExecutionReady {
    let store_authority =
        BlobLifecycleStoreAuthority::from_current_store_authority(current_authority(case));
    let lowering = store_authority.lowering_capability();
    let readiness =
        BlobLifecycleReadinessAuthority::from_s6_placement_seed(s6_placement_admission_seed(case));
    BlobLifecycleAdmission::start(declaration(case))
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(scoped_chunk(case))
        .success("reachability should admit")
        .admit_placement(&readiness)
        .ready_for_execution(readiness)
        .success("readiness should admit")
}

fn scoped_chunk(case: &str) -> ScopedBlobChunk {
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    ScopedBlobChunk::from_integrity_proof(integrity_proof_for_scope(
        scope,
        chunk_digest(case).as_str().as_bytes(),
    ))
}

fn declaration(case: &str) -> BlobLifecycleDeclaration {
    declaration_with_stored_digest(case, scoped_chunk(case).stored_digest().digest().clone())
}

fn declaration_with_stored_digest(
    case: &str,
    stored_digest: StableDigest,
) -> BlobLifecycleDeclaration {
    BlobLifecycleDeclaration::new(
        BlobObjectId::from_declared_digest(digest(&format!("sha256:{case}-object"))),
        BlobGeneration::published(1),
        ChunkTreeRoot::from_declared_digest(digest(&format!("sha256:{case}-root"))),
        LogicalContentDigest::from_declared_digest(digest(&format!("sha256:{case}-logical"))),
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

fn s6_placement_admission_seed(case: &str) -> S6ClosedS7PlacementAdmissionSeed {
    let s6_case = format!("{case}.s6-placement");
    admit_s6_s7_placement_handoff(
        S6S7PlacementAdmissionAuthority::from_current_store_authority(current_authority(&s6_case)),
    )
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
