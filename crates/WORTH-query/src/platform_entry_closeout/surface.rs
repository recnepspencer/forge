use super::alignment::{
    docs_coverage_alignment_audit, inventory_alignment_audit, WorthQueryPlatformEntryAlignmentAudit,
};
use super::compile_fail::{
    worth_query_platform_entry_compile_fail_boundary_digest,
    WorthQueryPlatformEntryCompileFailAudit,
};
use super::hostile::{
    worth_query_platform_entry_hostile_manifest, WorthQueryPlatformEntryHostileAudit,
};
use super::parity::{
    worth_query_platform_entry_parity_manifest, WorthQueryPlatformEntryParityAudit,
};
use crate::identity::hash_parts;
use crate::orchestration_inventory::WorthQueryOrchestrationSurfaceInventory;
use crate::public_doc_coverage::WorthQueryPublicDocCoverageInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPlatformEntryCloseoutSurface {
    public_surface_digest: String,
    docs_coverage_digest: String,
    compile_fail_boundary_digest: String,
    parity_digest: String,
    hostile_digest: String,
    inventory_alignment: WorthQueryPlatformEntryAlignmentAudit,
    docs_coverage_alignment: WorthQueryPlatformEntryAlignmentAudit,
    compile_fail_audit: WorthQueryPlatformEntryCompileFailAudit,
    parity_audit: WorthQueryPlatformEntryParityAudit,
    hostile_audit: WorthQueryPlatformEntryHostileAudit,
    closeout_surface_digest: String,
}

impl WorthQueryPlatformEntryCloseoutSurface {
    #[allow(clippy::too_many_arguments)]
    fn new(
        public_surface_digest: String,
        docs_coverage_digest: String,
        compile_fail_boundary_digest: String,
        parity_digest: String,
        hostile_digest: String,
        inventory_alignment: WorthQueryPlatformEntryAlignmentAudit,
        docs_coverage_alignment: WorthQueryPlatformEntryAlignmentAudit,
        compile_fail_audit: WorthQueryPlatformEntryCompileFailAudit,
        parity_audit: WorthQueryPlatformEntryParityAudit,
        hostile_audit: WorthQueryPlatformEntryHostileAudit,
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

    pub fn inventory_alignment(&self) -> &WorthQueryPlatformEntryAlignmentAudit {
        &self.inventory_alignment
    }

    pub fn docs_coverage_alignment(&self) -> &WorthQueryPlatformEntryAlignmentAudit {
        &self.docs_coverage_alignment
    }

    pub fn compile_fail_audit(&self) -> &WorthQueryPlatformEntryCompileFailAudit {
        &self.compile_fail_audit
    }

    pub fn parity_audit(&self) -> &WorthQueryPlatformEntryParityAudit {
        &self.parity_audit
    }

    pub fn hostile_audit(&self) -> &WorthQueryPlatformEntryHostileAudit {
        &self.hostile_audit
    }

    pub fn closeout_surface_digest(&self) -> &str {
        &self.closeout_surface_digest
    }
}

pub fn worth_query_platform_entry_closeout_surface() -> WorthQueryPlatformEntryCloseoutSurface {
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();
    let docs = WorthQueryPublicDocCoverageInventory::current();
    let parity = worth_query_platform_entry_parity_manifest();
    let hostile = worth_query_platform_entry_hostile_manifest();

    WorthQueryPlatformEntryCloseoutSurface::new(
        inventory.inventory_digest().to_string(),
        docs.coverage_digest().to_string(),
        worth_query_platform_entry_compile_fail_boundary_digest(),
        parity.parity_digest().to_string(),
        hostile.hostile_digest().to_string(),
        inventory_alignment_audit(),
        docs_coverage_alignment_audit(),
        WorthQueryPlatformEntryCompileFailAudit::current(),
        WorthQueryPlatformEntryParityAudit::current(),
        WorthQueryPlatformEntryHostileAudit::current(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closeout_surface_reuses_live_authority_digests_and_green_audits() {
        let surface = worth_query_platform_entry_closeout_surface();

        assert_eq!(
            surface.public_surface_digest(),
            WorthQueryOrchestrationSurfaceInventory::current().inventory_digest()
        );
        assert_eq!(
            surface.docs_coverage_digest(),
            WorthQueryPublicDocCoverageInventory::current().coverage_digest()
        );
        assert!(surface.inventory_alignment().is_aligned());
        assert!(surface.docs_coverage_alignment().is_aligned());
        assert!(surface.compile_fail_audit().missing_surfaces().is_empty());
        assert!(surface.parity_audit().missing_equivalence_rows().is_empty());
        assert!(surface.hostile_audit().missing_divergence_rows().is_empty());
        assert!(!surface.closeout_surface_digest().is_empty());
    }
}
