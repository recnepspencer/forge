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
    ForgeQueryGraphObligationAdoptionManifest, ForgeQueryGraphObligationAdoptionProof,
};
pub use consumer_declaration::ForgeQueryGraphObligationConsumerRegistrationDeclaration;
pub use error::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationConsumerKitErrorKind,
};
pub use in_memory_proof::{
    ForgeQueryGraphObligationExecutionProof, ForgeQueryGraphObligationExecutionProofRow,
    ForgeQueryGraphObligationInMemoryProof, ForgeQueryGraphObligationInMemorySelectedObligation,
    ForgeQueryGraphObligationInMemoryTestWorkspace,
};
pub use kit::{graph_obligation_consumer_kit, ForgeQueryGraphObligationConsumerKit};
pub use local_ceremony_audit::{
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationLocalCeremonyFinding,
};
pub use residue_manifest::{
    ForgeQueryGraphObligationResidueCertification, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow,
};
pub use selector_coverage::{
    ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ForgeQueryGraphObligationSelectorCoverageRow,
};
pub use support_pin::{
    ForgeQueryGraphObligationSupportPin, ForgeQueryGraphObligationSupportPinFinding,
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
    format!("forge-query-graph-obligation-{scope}:{hash:016x}")
}
