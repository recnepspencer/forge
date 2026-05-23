use crate::domain_capabilities::certification::{
    forge_query_domain_capability_compile_fail_boundaries,
    forge_query_domain_capability_compile_fail_boundary_digest,
    forge_query_domain_capability_golden_transcript_digest,
    forge_query_domain_capability_golden_transcripts,
    forge_query_domain_capability_public_surface_inventory,
    forge_query_domain_capability_target_dx_digest,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityCertificationSurface {
    public_surface_digest: String,
    target_dx_digest: String,
    golden_transcript_digest: String,
    compile_fail_boundary_digest: String,
    certification_surface_digest: String,
    category_count: usize,
    golden_transcript_count: usize,
    compile_fail_boundary_count: usize,
}

impl ForgeQueryDomainCapabilityCertificationSurface {
    pub(crate) fn new(
        public_surface_digest: String,
        target_dx_digest: String,
        golden_transcript_digest: String,
        compile_fail_boundary_digest: String,
        category_count: usize,
        golden_transcript_count: usize,
        compile_fail_boundary_count: usize,
    ) -> Self {
        let certification_surface_digest = hash_parts(&[
            "forge_query_domain_capability_certification_surface_v1".to_string(),
            format!("public-surface:{public_surface_digest}"),
            format!("target-dx:{target_dx_digest}"),
            format!("golden:{golden_transcript_digest}"),
            format!("compile-fail:{compile_fail_boundary_digest}"),
            format!("categories:{category_count}"),
            format!("golden-count:{golden_transcript_count}"),
            format!("compile-fail-count:{compile_fail_boundary_count}"),
        ]);
        Self {
            public_surface_digest,
            target_dx_digest,
            golden_transcript_digest,
            compile_fail_boundary_digest,
            certification_surface_digest,
            category_count,
            golden_transcript_count,
            compile_fail_boundary_count,
        }
    }

    pub fn public_surface_digest(&self) -> &str {
        &self.public_surface_digest
    }

    pub fn target_dx_digest(&self) -> &str {
        &self.target_dx_digest
    }

    pub fn golden_transcript_digest(&self) -> &str {
        &self.golden_transcript_digest
    }

    pub fn compile_fail_boundary_digest(&self) -> &str {
        &self.compile_fail_boundary_digest
    }

    pub fn certification_surface_digest(&self) -> &str {
        &self.certification_surface_digest
    }

    pub fn category_count(&self) -> usize {
        self.category_count
    }

    pub fn golden_transcript_count(&self) -> usize {
        self.golden_transcript_count
    }

    pub fn compile_fail_boundary_count(&self) -> usize {
        self.compile_fail_boundary_count
    }
}

pub fn forge_query_domain_capability_certification_surface(
) -> ForgeQueryDomainCapabilityCertificationSurface {
    let inventory = forge_query_domain_capability_public_surface_inventory();
    let golden = forge_query_domain_capability_golden_transcripts();
    let compile_fail = forge_query_domain_capability_compile_fail_boundaries();

    ForgeQueryDomainCapabilityCertificationSurface::new(
        inventory.public_surface_digest(),
        forge_query_domain_capability_target_dx_digest(),
        forge_query_domain_capability_golden_transcript_digest(),
        forge_query_domain_capability_compile_fail_boundary_digest(),
        inventory.rows().len(),
        golden.len(),
        compile_fail.len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certification_surface_reuses_public_inventory_and_boundary_manifests() {
        let inventory = forge_query_domain_capability_public_surface_inventory();
        let surface = forge_query_domain_capability_certification_surface();

        assert_eq!(
            surface.public_surface_digest(),
            inventory.public_surface_digest()
        );
        assert_eq!(
            surface.target_dx_digest(),
            forge_query_domain_capability_target_dx_digest()
        );
        assert_eq!(
            surface.golden_transcript_digest(),
            forge_query_domain_capability_golden_transcript_digest()
        );
        assert_eq!(
            surface.compile_fail_boundary_digest(),
            forge_query_domain_capability_compile_fail_boundary_digest()
        );
        assert_eq!(surface.category_count(), inventory.rows().len());
        assert_eq!(
            surface.golden_transcript_count(),
            forge_query_domain_capability_golden_transcripts().len()
        );
        assert_eq!(
            surface.compile_fail_boundary_count(),
            forge_query_domain_capability_compile_fail_boundaries().len()
        );
        assert!(!surface.certification_surface_digest().is_empty());
    }

    #[test]
    fn certification_surface_keeps_target_dx_and_boundary_digests_distinct() {
        let surface = forge_query_domain_capability_certification_surface();

        assert_ne!(
            surface.target_dx_digest(),
            surface.golden_transcript_digest()
        );
        assert_ne!(
            surface.target_dx_digest(),
            surface.compile_fail_boundary_digest()
        );
        assert_ne!(
            surface.golden_transcript_digest(),
            surface.compile_fail_boundary_digest()
        );
    }
}
