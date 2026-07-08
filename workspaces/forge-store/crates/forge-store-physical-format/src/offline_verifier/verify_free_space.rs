use super::codec::DecodedOfflineManifestSections;
use crate::{
    ManifestDiscoveryAuthority, ManifestDiscoveryReport, OfflineVerifierCounterSnapshot,
    OfflineVerifierDenial, OfflineVerifierDenialKind, PhysicalReference,
    PhysicalReferenceAuthority,
};

pub(crate) struct FreeSpaceVerificationContext {
    pub references: PhysicalReferenceAuthority,
    pub manifests: ManifestDiscoveryAuthority,
}

pub(crate) fn verify_all_free_space(
    ctx: &FreeSpaceVerificationContext,
    manifest_report: ManifestDiscoveryReport<'_>,
    decoded: &DecodedOfflineManifestSections,
    counters: OfflineVerifierCounterSnapshot,
    discovered: &mut Vec<PhysicalReference>,
) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
    let mut counters = counters;
    for entry in &decoded.free_space {
        counters = counters.with_free_space_entry_checked();
        let admission = ctx.references.admit_free_space_reuse(entry.reuse_cell());
        verify_free_space_manifest_membership(ctx.manifests, manifest_report, admission, counters)?;
        discovered.push(admission.reference());
    }
    Ok(counters)
}

fn verify_free_space_manifest_membership(
    manifests: ManifestDiscoveryAuthority,
    manifest_report: ManifestDiscoveryReport<'_>,
    admission: crate::PhysicalReferenceAdmissionWitness,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    manifests
        .validate_free_space_reuse(manifest_report, admission)
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::ManifestDiscoveryDenied, counters)
                .with_manifest_denial(denial)
        })?;
    Ok(())
}
