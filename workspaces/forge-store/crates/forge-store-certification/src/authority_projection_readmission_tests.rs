use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{
    compare_retained_store_authority_evidence,
    deny_lower_authority_source_readmission_as_current_authority,
    deny_unsupported_authority_source_readmission_as_current_authority,
    readmit_external_store_authority_token, readmit_retained_store_authority_evidence,
    report_derived_store_authority_evidence, report_retained_store_authority_evidence,
    require_current_physical_authority, require_current_store_authority,
    StoreAuthorityReadmissionDenial, StoreDerivedAuthorityEvidenceRole,
    StoreExternalAuthorityToken, StoreLowerAuthoritySource,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

#[test]
fn store_external_tokens_require_owner_readmission() {
    let current_authority = require_current_store_authority(boundary_fact(
        "store.phase8.current.identity",
        "segment-0001",
    ));
    let owner_validated_token =
        StoreExternalAuthorityToken::imported(current_authority.identity().aspect_key().as_str());

    let readmitted =
        match readmit_external_store_authority_token(owner_validated_token, &current_authority) {
            TransitionOutcome::Success(witness) => witness,
            outcome => panic!("owner-validated external token should readmit: {outcome:?}"),
        };

    assert_eq!(readmitted.identity(), current_authority.identity());
    assert_eq!(
        readmitted.physical_witness(),
        current_authority.physical_witness()
    );
}

#[test]
fn store_external_token_mismatch_denies_current_authority() {
    let current_authority = require_current_store_authority(boundary_fact(
        "store.phase8.current.identity",
        "segment-0002",
    ));
    let mismatched_token = StoreExternalAuthorityToken::imported("store.phase8.other.identity");

    let denial = match readmit_external_store_authority_token(mismatched_token, &current_authority)
    {
        TransitionOutcome::Denied(denial) => denial,
        outcome => panic!("mismatched external token should deny: {outcome:?}"),
    };

    assert_eq!(
        denial,
        StoreAuthorityReadmissionDenial::ExternalTokenMismatch {
            external_token_text: "store.phase8.other.identity".into(),
            current_identity_text: "store.phase8.current.identity".into(),
        }
    );
}

#[test]
fn stale_external_token_denies_current_authority() {
    let current_authority = require_current_store_authority(boundary_fact(
        "store.phase8.stale.identity",
        "segment-0005",
    ));
    let stale_token = StoreExternalAuthorityToken::stale_retained(
        current_authority.identity().aspect_key().as_str(),
    );

    let denial = match readmit_external_store_authority_token(stale_token, &current_authority) {
        TransitionOutcome::Denied(denial) => denial,
        outcome => panic!("stale external token should deny: {outcome:?}"),
    };

    assert_eq!(
        denial,
        StoreAuthorityReadmissionDenial::StaleExternalToken {
            external_token_text: "store.phase8.stale.identity".into(),
        }
    );
}

#[test]
fn retained_evidence_remains_retained_evidence_until_readmitted() {
    let current_authority = require_current_store_authority(boundary_fact(
        "store.phase8.retained.identity",
        "segment-0003",
    ));
    let retained = report_retained_store_authority_evidence(&current_authority);
    let comparison = compare_retained_store_authority_evidence(&retained, &retained);
    let current_physical = require_current_physical_authority(&current_authority);

    assert!(comparison.proves_same_retained_authority());
    assert_eq!(current_physical.identity(), retained.identity());

    let readmitted = match readmit_retained_store_authority_evidence(retained, &current_authority) {
        TransitionOutcome::Success(witness) => witness,
        outcome => panic!("matching retained evidence should readmit: {outcome:?}"),
    };

    assert_eq!(readmitted.identity(), current_authority.identity());
}

#[test]
fn derived_authority_evidence_has_lower_authority_role() {
    let current_authority = require_current_store_authority(boundary_fact(
        "store.phase8.derived.identity",
        "segment-0004",
    ));
    let derived = report_derived_store_authority_evidence(
        &current_authority,
        StoreDerivedAuthorityEvidenceRole::DigestProjection,
    );

    assert_eq!(derived.identity(), current_authority.identity());
    assert_eq!(
        derived.role(),
        StoreDerivedAuthorityEvidenceRole::DigestProjection
    );
}

#[test]
fn lower_authority_source_readmission_returns_structured_denial() {
    let denial = match deny_lower_authority_source_readmission_as_current_authority(
        StoreLowerAuthoritySource::DigestText,
    ) {
        TransitionOutcome::Denied(denial) => denial,
        outcome => panic!("lower-authority source should deny readmission: {outcome:?}"),
    };

    assert_eq!(
        denial,
        StoreAuthorityReadmissionDenial::LowerAuthoritySourceRequiresOwnerReadmission {
            source: StoreLowerAuthoritySource::DigestText,
        }
    );
}

#[test]
fn unsupported_lower_authority_source_readmission_returns_structured_denial() {
    let denial = match deny_unsupported_authority_source_readmission_as_current_authority(
        StoreLowerAuthoritySource::TerminalProjectionText,
    ) {
        TransitionOutcome::Denied(denial) => denial,
        outcome => panic!("unsupported source should deny readmission: {outcome:?}"),
    };

    assert_eq!(
        denial,
        StoreAuthorityReadmissionDenial::UnsupportedAuthoritySource {
            source: StoreLowerAuthoritySource::TerminalProjectionText,
        }
    );
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
