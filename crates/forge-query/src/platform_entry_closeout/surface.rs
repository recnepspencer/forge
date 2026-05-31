use super::alignment::{
    docs_coverage_alignment_audit, inventory_alignment_audit, ForgeQueryPlatformEntryAlignmentAudit,
};
use super::compile_fail::{
    forge_query_platform_entry_compile_fail_boundary_digest,
    ForgeQueryPlatformEntryCompileFailAudit,
};
use super::hostile::{
    forge_query_platform_entry_hostile_manifest, ForgeQueryPlatformEntryHostileAudit,
};
use super::parity::{
    forge_query_platform_entry_parity_manifest, ForgeQueryPlatformEntryParityAudit,
};
use crate::identity::hash_parts;
use crate::orchestration_inventory::ForgeQueryOrchestrationSurfaceInventory;
use crate::public_doc_coverage::ForgeQueryPublicDocCoverageInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryCloseoutSurface {
    public_surface_digest: String,
    docs_coverage_digest: String,
    compile_fail_boundary_digest: String,
    parity_digest: String,
    hostile_digest: String,
    inventory_alignment: ForgeQueryPlatformEntryAlignmentAudit,
    docs_coverage_alignment: ForgeQueryPlatformEntryAlignmentAudit,
    compile_fail_audit: ForgeQueryPlatformEntryCompileFailAudit,
    parity_audit: ForgeQueryPlatformEntryParityAudit,
    hostile_audit: ForgeQueryPlatformEntryHostileAudit,
    closeout_surface_digest: String,
}

impl ForgeQueryPlatformEntryCloseoutSurface {
    #[allow(clippy::too_many_arguments)]
    fn new(
        public_surface_digest: String,
        docs_coverage_digest: String,
        compile_fail_boundary_digest: String,
        parity_digest: String,
        hostile_digest: String,
        inventory_alignment: ForgeQueryPlatformEntryAlignmentAudit,
        docs_coverage_alignment: ForgeQueryPlatformEntryAlignmentAudit,
        compile_fail_audit: ForgeQueryPlatformEntryCompileFailAudit,
        parity_audit: ForgeQueryPlatformEntryParityAudit,
        hostile_audit: ForgeQueryPlatformEntryHostileAudit,
    ) -> Self {
        let closeout_surface_digest = hash_parts(&[
            public_surface_digest.clone(),
            docs_coverage_digest.clone(),
            compile_fail_boundary_digest.clone(),
            parity_digest.clone(),
            hostile_digest.clone(),
            inventory_alignment.digest().to_string(),
            docs_coverage_alignment.digest().to_string(),
        ]);
        Self {
            public_surface_digest,
            docs_coverage_digest,
            compile_fail_boundary_digest,
            parity_digest,
            hostile_digest,
            inventory_alignment,
            docs_coverage_alignment,
            compile_fail_audit,
            parity_audit,
            hostile_audit,
            closeout_surface_digest,
        }
    }

    pub fn public_surface_digest(&self) -> &str {
        &self.public_surface_digest
    }

    pub fn docs_coverage_digest(&self) -> &str {
        &self.docs_coverage_digest
    }

    pub fn compile_fail_boundary_digest(&self) -> &str {
        &self.compile_fail_boundary_digest
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn hostile_digest(&self) -> &str {
        &self.hostile_digest
    }

    pub fn inventory_alignment(&self) -> &ForgeQueryPlatformEntryAlignmentAudit {
        &self.inventory_alignment
    }

    pub fn docs_coverage_alignment(&self) -> &ForgeQueryPlatformEntryAlignmentAudit {
        &self.docs_coverage_alignment
    }

    pub fn compile_fail_audit(&self) -> &ForgeQueryPlatformEntryCompileFailAudit {
        &self.compile_fail_audit
    }

    pub fn parity_audit(&self) -> &ForgeQueryPlatformEntryParityAudit {
        &self.parity_audit
    }

    pub fn hostile_audit(&self) -> &ForgeQueryPlatformEntryHostileAudit {
        &self.hostile_audit
    }

    pub fn closeout_surface_digest(&self) -> &str {
        &self.closeout_surface_digest
    }
}

pub fn forge_query_platform_entry_closeout_surface() -> ForgeQueryPlatformEntryCloseoutSurface {
    let inventory = ForgeQueryOrchestrationSurfaceInventory::current();
    let docs = ForgeQueryPublicDocCoverageInventory::current();
    let parity = forge_query_platform_entry_parity_manifest();
    let hostile = forge_query_platform_entry_hostile_manifest();

    ForgeQueryPlatformEntryCloseoutSurface::new(
        inventory.inventory_digest().to_string(),
        docs.coverage_digest().to_string(),
        forge_query_platform_entry_compile_fail_boundary_digest(),
        parity.parity_digest().to_string(),
        hostile.hostile_digest().to_string(),
        inventory_alignment_audit(),
        docs_coverage_alignment_audit(),
        ForgeQueryPlatformEntryCompileFailAudit::current(),
        ForgeQueryPlatformEntryParityAudit::current(),
        ForgeQueryPlatformEntryHostileAudit::current(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closeout_surface_reuses_live_authority_digests_and_green_audits() {
        let surface = forge_query_platform_entry_closeout_surface();

        assert_eq!(
            surface.public_surface_digest(),
            ForgeQueryOrchestrationSurfaceInventory::current().inventory_digest()
        );
        assert_eq!(
            surface.docs_coverage_digest(),
            ForgeQueryPublicDocCoverageInventory::current().coverage_digest()
        );
        assert!(surface.inventory_alignment().is_aligned());
        assert!(surface.docs_coverage_alignment().is_aligned());
        assert!(surface.compile_fail_audit().missing_surfaces().is_empty());
        assert!(surface.parity_audit().missing_equivalence_rows().is_empty());
        assert!(surface.hostile_audit().missing_divergence_rows().is_empty());
        assert!(!surface.closeout_surface_digest().is_empty());
    }
}
