mod assembly;
mod authority;
mod binding;
mod counters;
mod denial;
mod digest_basis;
mod evidence_reference;
mod explanation_envelope;
mod identity;
mod receipt;
pub(super) mod retained_mapping;

pub use assembly::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalInspectionAdmissionSummary,
    BridgeCausalInspectionAdmissionSummaryKind,
};
pub use authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
pub use binding::{BridgeCausalEvidenceBinding, BridgeCausalEvidenceBindingClass};
pub use counters::BridgeCausalEnvelopeCounters;
pub use denial::{BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind};
pub use evidence_reference::{
    BridgeCausalEvidenceReference, BridgeCausalEvidenceReferenceIdentity,
};
pub use explanation_envelope::BridgeCausalExplanationEnvelope;
pub use identity::BridgeCausalEnvelopeIdentity;
pub use receipt::BridgeCausalEnvelopeReceipt;

use digest_basis::BridgeCausalEnvelopeDigestArtifact;

fn causal_envelope_digest(artifact: BridgeCausalEnvelopeDigestArtifact, parts: &[&str]) -> String {
    use sha2::{Digest, Sha256};

    let digest_domain = artifact.digest_domain();
    let mut canonical = String::from(digest_domain);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{digest_domain}:sha256:{digest:x}")
}
