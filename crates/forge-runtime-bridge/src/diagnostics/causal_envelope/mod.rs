mod assembly;
mod authority;
mod binding;
mod counters;
mod denial;
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
pub use evidence_reference::BridgeCausalEvidenceReference;
pub use explanation_envelope::BridgeCausalExplanationEnvelope;
pub use identity::BridgeCausalEnvelopeIdentity;
pub use receipt::BridgeCausalEnvelopeReceipt;

fn digest(label: &str, parts: &[&str]) -> String {
    use sha2::{Digest, Sha256};

    let mut canonical = String::from(label);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{label}:sha256:{digest:x}")
}
