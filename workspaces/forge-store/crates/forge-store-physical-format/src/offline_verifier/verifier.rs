use super::manifest_membership::{build_root_manifest, verify_membership_posture};
use super::manifest_pipeline::{
    collect_layout_inspection_counters, construct_minimal_verifier_report,
    decode_manifest_sections, reject_backend_residue, select_single_root_manifest,
    verify_manifest_discovery,
};
use super::verify_extents::{verify_all_extents, ExtentVerificationContext};
use super::verify_free_space::{verify_all_free_space, FreeSpaceVerificationContext};
use super::verify_pages::{verify_all_pages, PageVerificationContext};
use crate::{
    ManifestDiscoveryAuthority, MinimalManifestVerifierReport, OfflineVerifierDenial,
    PersistedPhysicalLayout, PhysicalHeaderAuthority, PhysicalReferenceAuthority,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflinePhysicalVerifier {
    headers: PhysicalHeaderAuthority,
    references: PhysicalReferenceAuthority,
    manifests: ManifestDiscoveryAuthority,
}

impl OfflinePhysicalVerifier {
    pub const fn for_canonical_physical_format(headers: PhysicalHeaderAuthority) -> Self {
        Self {
            headers,
            references: PhysicalReferenceAuthority::for_canonical_physical_format(),
            manifests: ManifestDiscoveryAuthority::for_canonical_physical_format(),
        }
    }

    pub fn verify(
        &self,
        layout: &PersistedPhysicalLayout,
    ) -> Result<MinimalManifestVerifierReport, OfflineVerifierDenial> {
        let counters = collect_layout_inspection_counters(layout);
        let root_manifest = select_single_root_manifest(layout, counters)?;
        let decoded =
            decode_manifest_sections(self.headers.byte_order(), layout, root_manifest, counters)?;
        let counters = counters.with_manifest_rows_decoded(decoded.decoded_rows);
        verify_membership_posture(&decoded, counters)?;
        let root = build_root_manifest(&decoded);
        let manifest_report = verify_manifest_discovery(
            self.manifests,
            self.references,
            &root,
            decoded.root,
            counters,
        )?;
        reject_backend_residue(layout, manifest_report, self.manifests, counters)?;
        let mut discovered = vec![self
            .references
            .admit_root_publication(decoded.root)
            .reference()];
        let page_ctx = PageVerificationContext {
            headers: &self.headers,
            references: self.references,
            manifests: self.manifests,
        };
        let counters = verify_all_pages(
            &page_ctx,
            layout.pages(),
            manifest_report,
            &decoded,
            counters,
            &mut discovered,
        )?;
        let extent_ctx = ExtentVerificationContext {
            headers: &self.headers,
            references: self.references,
            manifests: self.manifests,
        };
        let counters = verify_all_extents(
            &extent_ctx,
            layout.extents(),
            manifest_report,
            &decoded,
            counters,
            &mut discovered,
        )?;
        let free_space_ctx = FreeSpaceVerificationContext {
            references: self.references,
            manifests: self.manifests,
        };
        let counters = verify_all_free_space(
            &free_space_ctx,
            manifest_report,
            &decoded,
            counters,
            &mut discovered,
        )?;
        construct_minimal_verifier_report(&decoded, discovered, counters)
    }
}
