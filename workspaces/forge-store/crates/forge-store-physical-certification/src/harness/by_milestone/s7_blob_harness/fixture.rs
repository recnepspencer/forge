use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, AuthoritativeRecordAspectStateArtifact,
    ContractValidatedAspectArtifact, ContractValidationInput, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_contracts::{
    StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
};

use super::foundational_profile::BlobHarnessMaterializedProfile;
use super::scenario_seed::BlobHarnessScenarioSeed;

pub(super) fn blob_harness_seed_fixture(
    seed: &BlobHarnessScenarioSeed,
    materialized_profile: &BlobHarnessMaterializedProfile,
) -> StoreAspectBoundaryFact {
    let aspect_key = aspects()
        .vocabulary()
        .key("store.s7.blob.harness.seed")
        .unwrap();
    let contract = scalar_string_contract(aspect_key.clone());
    let value = AspectValue::String(InternedString::from(seed_fixture_token(
        seed,
        materialized_profile,
    )));
    let validated = validate_native_value(&contract, ContractValidationInput::from(value));
    let state = admit_authoritative_state(validated);
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(aspect_key),
        StoreAspectAuthorityInput::new(state, physical_witness()),
    )
    .expect("static S.7 blob harness fixture is admitted")
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(87))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validate_native_value(
    contract: &AspectContract,
    value: ContractValidationInput,
) -> ContractValidatedAspectArtifact {
    match aspects().validate().against(contract).value(value) {
        TransitionOutcome::Success(validated) => validated,
        outcome => panic!("S.7 blob harness seed fixture should validate: {outcome:?}"),
    }
}

fn admit_authoritative_state(
    validated_value: ContractValidatedAspectArtifact,
) -> AuthoritativeRecordAspectStateArtifact {
    match aspects().authoritative_state().admit([validated_value]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("S.7 blob harness seed fixture should admit: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary_instance(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
            ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
        )
        .expect("static aspect-native witness scope is valid"),
    )
    .expect("static aspect-native physical witness is valid")
}

fn seed_fixture_token(
    seed: &BlobHarnessScenarioSeed,
    materialized_profile: &BlobHarnessMaterializedProfile,
) -> String {
    format!(
        "profile={:?};profile_identity={};size={:?};chunk={:?};placement={:?};scope={:?};access={:?};failure={:?};actors={:?};chunks={};bytes={}",
        seed.profile(),
        profile_identity_digest_hex(materialized_profile),
        seed.size_class(),
        seed.chunk_size_class(),
        seed.placement_class(),
        seed.security_scope(),
        seed.access_mode(),
        seed.failure_point(),
        seed.actor_mix(),
        seed.topology().chunk_count(),
        seed.topology().logical_bytes(),
    )
}

fn profile_identity_digest_hex(materialized_profile: &BlobHarnessMaterializedProfile) -> String {
    materialized_profile
        .foundational_identity()
        .digest()
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
