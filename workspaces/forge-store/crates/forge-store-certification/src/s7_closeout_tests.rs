use crate::{
    admit_s7_backup_non_claim_handoff, admit_s7_full_certification_non_claim_handoff,
    admit_s7_key_lifecycle_non_claim_handoff, admit_s7_layout_readiness_handoff,
    evaluate_s7_closeout_request, S7CloseoutCertificationInput, S7CloseoutDenial,
    S7CloseoutEvidencePolicy, S7CloseoutRequest, S7CloseoutShortcutAttempt,
    S7CloseoutShortcutInput,
};
use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_physical_certification::{
    s7_blob_harness_closeout_sources_for_seed, BlobHarnessProfile, BlobHarnessScenarioSeed,
};
use forge_store_readiness::{
    admit_s6_s7_placement_handoff, S10BackupRepairReadinessNonClaim,
    S11KeyLifecycleReadinessNonClaim, S12FullCertificationNonClaim,
    S6S7PlacementAdmissionAuthority, S8LayoutReadinessNonClaim,
};

#[test]
fn s7_closeout_binds_executed_sources_and_downstream_non_claims() {
    let sources = s7_blob_harness_closeout_sources_for_seed(heavy_seed()).unwrap();
    let closeout = evaluate_s7_closeout_request(S7CloseoutRequest::Canonical(
        S7CloseoutCertificationInput::from_executed_sources(
            sources,
            S7CloseoutEvidencePolicy::counter_backed_foundational(),
        ),
    ))
    .unwrap();

    let layout = admit_s7_layout_readiness_handoff(&closeout).unwrap();
    let backup = admit_s7_backup_non_claim_handoff(&closeout).unwrap();
    let keys = admit_s7_key_lifecycle_non_claim_handoff(&closeout).unwrap();
    let full = admit_s7_full_certification_non_claim_handoff(&closeout).unwrap();

    assert!(closeout
        .materialized_evidence()
        .proof_summary()
        .checked_execution());
    assert!(!closeout.binding_tag().is_empty());
    assert_eq!(layout.non_claims(), &S8LayoutReadinessNonClaim::required());
    assert_eq!(
        backup.non_claims(),
        &S10BackupRepairReadinessNonClaim::required()
    );
    assert_eq!(
        keys.non_claims(),
        &S11KeyLifecycleReadinessNonClaim::required()
    );
    assert_eq!(full.non_claims(), &S12FullCertificationNonClaim::required());
}

#[test]
fn s7_closeout_shortcut_attempts_are_explicit_runtime_denials() {
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::CopiedReceipt,
        S7CloseoutShortcutAttempt::CopiedReceipt,
    );
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::CopiedChunkRows { row_count: 3 },
        S7CloseoutShortcutAttempt::CopiedChunkRows,
    );
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::CopiedProofId {
            proof_id: "proof-1".to_owned(),
        },
        S7CloseoutShortcutAttempt::CopiedProofId,
    );
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::S6PlacementReadinessOnly {
            seed: admit_s6_s7_placement_handoff(
                S6S7PlacementAdmissionAuthority::from_current_store_authority(current_authority(
                    "store.s7.closeout.shortcut",
                )),
            ),
        },
        S7CloseoutShortcutAttempt::S6PlacementReadinessOnly,
    );
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::S5FutureChunkPlaceholderOnly {
            label: "future-chunk".to_owned(),
        },
        S7CloseoutShortcutAttempt::S5FutureChunkPlaceholderOnly,
    );
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::TerminalProjectionOnly,
        S7CloseoutShortcutAttempt::TerminalProjectionOnly,
    );
    assert_shortcut_rejected(
        S7CloseoutShortcutInput::RawCountersOnly { row_count: 9 },
        S7CloseoutShortcutAttempt::RawCountersOnly,
    );
}

fn assert_shortcut_rejected(
    shortcut: S7CloseoutShortcutInput,
    expected_attempt: S7CloseoutShortcutAttempt,
) {
    let denial = evaluate_s7_closeout_request(S7CloseoutRequest::Shortcut(shortcut))
        .expect_err("shortcut must deny");
    match denial {
        S7CloseoutDenial::ShortcutRejected(report) => {
            assert_eq!(report.attempt(), expected_attempt);
            assert!(!report.reason().is_empty());
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

fn heavy_seed() -> BlobHarnessScenarioSeed {
    BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::heavy_multi_gb())
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .seed_actor_mix()
        .build()
        .unwrap()
}

fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current"))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> forge_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
