mod adoption_manifest;
mod consumer_declaration;
mod error;
mod in_memory_proof;
mod kit;
mod local_ceremony_audit;
mod residue_manifest;
mod selector_coverage;
mod support_pin;

#[cfg(test)]
mod tests;

pub use adoption_manifest::{
    WorthQueryGraphObligationAdoptionManifest, WorthQueryGraphObligationAdoptionProof,
    WorthQueryGraphObligationExecutionBackedAdoptionProof,
};
pub use consumer_declaration::WorthQueryGraphObligationConsumerRegistrationDeclaration;
pub use error::{
    WorthQueryGraphObligationConsumerKitError, WorthQueryGraphObligationConsumerKitErrorKind,
};
pub use in_memory_proof::{
    WorthQueryGraphObligationExecutionProof, WorthQueryGraphObligationExecutionProofRow,
    WorthQueryGraphObligationInMemoryProof, WorthQueryGraphObligationInMemorySelectedObligation,
    WorthQueryGraphObligationInMemoryTestWorkspace,
};
pub use kit::{graph_obligation_consumer_kit, WorthQueryGraphObligationConsumerKit};
pub use local_ceremony_audit::{
    WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationLocalCeremonyFinding,
};
pub use residue_manifest::{
    WorthQueryGraphObligationResidueCertification, WorthQueryGraphObligationResidueManifest,
    WorthQueryGraphObligationResidueRow,
};
pub use selector_coverage::{
    WorthQueryGraphObligationSelectorCoverageDeclaration,
    WorthQueryGraphObligationSelectorCoverageRow,
};
pub use support_pin::{
    WorthQueryGraphObligationSupportPin, WorthQueryGraphObligationSupportPinFinding,
};

pub(super) fn kit_digest<'a>(scope: &str, parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in scope.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for part in parts {
        hash ^= u64::from(b'|');
        hash = hash.wrapping_mul(0x100000001b3);
        for byte in part.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("worth-query-graph-obligation-{scope}:{hash:016x}")
}
