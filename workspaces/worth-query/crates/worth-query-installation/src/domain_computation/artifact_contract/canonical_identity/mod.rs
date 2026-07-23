mod carriage_and_governance;
mod semantic_postures;
mod vocabulary;

use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_computation::WorthQueryArtifactContentIdentityContract;

use super::{WorthQueryArtifactContractIdentity, WorthQueryPortableArtifactContract};

pub(crate) fn canonical_artifact_contract_identity(
    contract: &WorthQueryPortableArtifactContract,
) -> WorthQueryArtifactContractIdentity {
    let mut hash = Sha256::new();
    hash_text_field(&mut hash, "family", contract.family.as_str());
    hash_text_field(
        &mut hash,
        "schema-version",
        &contract.schema_version.get().to_string(),
    );
    hash_text_field(
        &mut hash,
        "protocol-version",
        &contract.protocol_version.get().to_string(),
    );
    hash_content_identity(&mut hash, &contract.content_identity);
    vocabulary::hash_optional(
        &mut hash,
        "payload-owner",
        contract.ownership.payload_owner(),
    );
    vocabulary::hash_optional(
        &mut hash,
        "provider-family",
        contract.ownership.provider_family(),
    );
    semantic_postures::hash_occurrence_and_evidence(&mut hash, contract);
    semantic_postures::hash_reproducibility(&mut hash, &contract.reproducibility);
    semantic_postures::hash_search(&mut hash, &contract.search);
    semantic_postures::hash_convergence(&mut hash, &contract.convergence);
    semantic_postures::hash_transformation(&mut hash, &contract.transformation);
    carriage_and_governance::hash_carriage(&mut hash, contract);
    carriage_and_governance::hash_governance(&mut hash, contract);
    carriage_and_governance::hash_compatibility(&mut hash, &contract.compatibility);
    for role in &contract.producer_roles {
        hash_text_field(&mut hash, "producer-role", role);
    }
    for role in &contract.consumer_roles {
        hash_text_field(&mut hash, "consumer-role", role);
    }
    WorthQueryArtifactContractIdentity::minted(format!("{:x}", hash.finalize()))
}

fn hash_content_identity(hash: &mut Sha256, identity: &WorthQueryArtifactContentIdentityContract) {
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
