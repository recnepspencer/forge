use serde::Serialize;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{ConflictOverlapIdentity, ConflictPriorProofInput};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ConflictRoutingPosture {
    RequiresFamilySelection,
    ProvenIndependent,
    SerializableOnly,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConflictRoutingContract {
    overlap_identity: ConflictOverlapIdentity,
    prior_proof: ConflictPriorProofInput,
    posture: ConflictRoutingPosture,
    contract_digest: String,
}

impl ConflictRoutingContract {
    pub const fn overlap_identity(&self) -> &ConflictOverlapIdentity {
        &self.overlap_identity
    }

    pub const fn prior_proof(&self) -> &ConflictPriorProofInput {
        &self.prior_proof
    }

    pub const fn posture(&self) -> ConflictRoutingPosture {
        self.posture
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

pub fn admit_conflict_routing_contract(
    overlap_identity: ConflictOverlapIdentity,
    prior_proof: ConflictPriorProofInput,
    posture: ConflictRoutingPosture,
) -> ConflictRoutingContract {
    let mut parts = vec![
        "worth-schema:touched-graph-conflict-routing-contract:v1".to_string(),
        format!("overlap:{}", overlap_identity.overlap_identity_digest()),
        format!("posture:{posture:?}"),
    ];
    parts.extend(
        prior_proof
            .digest_parts()
            .into_iter()
            .map(|part| format!("prior-proof:{part}")),
    );
    let contract_digest = truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts);
    ConflictRoutingContract {
        overlap_identity,
        prior_proof,
        posture,
        contract_digest,
    }
}
