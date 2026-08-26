mod carriage_and_governance;
mod semantic_postures;
mod vocabulary;

use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::{
    hash_text_field, CanonicalHashByteCounter, CanonicalHashSink,
};
use crate::domain_computation::WorthQueryArtifactContentIdentityContract;

use super::{WorthQueryArtifactContractIdentity, WorthQueryPortableArtifactContract};

pub(crate) fn canonical_artifact_contract_identity(
    contract: &WorthQueryPortableArtifactContract,
) -> WorthQueryArtifactContractIdentity {
    let mut hash = Sha256::new();
    append_artifact_contract(&mut hash, contract);
    WorthQueryArtifactContractIdentity::minted(format!("{:x}", hash.finalize()))
}

pub(crate) fn canonical_artifact_contract_encoded_bytes(
    contract: &WorthQueryPortableArtifactContract,
) -> u64 {
    let mut counter = CanonicalHashByteCounter::default();
    append_artifact_contract(&mut counter, contract);
    counter.bytes()
}

fn append_artifact_contract(
    hash: &mut impl CanonicalHashSink,
    contract: &WorthQueryPortableArtifactContract,
) {
    hash_text_field(hash, "family", contract.family.as_str());
    hash_text_field(
        hash,
        "schema-version",
        &contract.schema_version.get().to_string(),
    );
    hash_text_field(
        hash,
        "protocol-version",
        &contract.protocol_version.get().to_string(),
    );
    hash_content_identity(hash, &contract.content_identity);
    vocabulary::hash_optional(hash, "payload-owner", contract.ownership.payload_owner());
    vocabulary::hash_optional(
        hash,
        "provider-family",
        contract.ownership.provider_family(),
    );
    semantic_postures::hash_occurrence_and_evidence(hash, contract);
    semantic_postures::hash_reproducibility(hash, &contract.reproducibility);
    semantic_postures::hash_search(hash, &contract.search);
    semantic_postures::hash_convergence(hash, &contract.convergence);
    semantic_postures::hash_transformation(hash, &contract.transformation);
    crate::domain_computation::hash_artifact_access_path(hash, &contract.access_path);
    carriage_and_governance::hash_carriage(hash, contract);
    crate::domain_computation::hash_decision_record_contract(hash, &contract.decisions);
    carriage_and_governance::hash_governance(hash, contract);
    carriage_and_governance::hash_compatibility(hash, &contract.compatibility);
    for role in &contract.producer_roles {
        hash_text_field(hash, "producer-role", role);
    }
    for role in &contract.consumer_roles {
        hash_text_field(hash, "consumer-role", role);
    }
}

fn hash_content_identity(
    hash: &mut impl CanonicalHashSink,
    identity: &WorthQueryArtifactContentIdentityContract,
) {
    match identity {
        WorthQueryArtifactContentIdentityContract::OwnerCanonicalProjection {
            projection_family,
            rule_version,
        } => {
            hash_text_field(hash, "content-identity", "owner-canonical-projection");
            hash_text_field(hash, "canonical-projection-family", projection_family);
            hash_text_field(hash, "canonical-rule", rule_version.as_str());
        }
        WorthQueryArtifactContentIdentityContract::CallerDigestDefined => {
            hash_text_field(hash, "content-identity", "caller-digest-defined");
        }
    }
}
