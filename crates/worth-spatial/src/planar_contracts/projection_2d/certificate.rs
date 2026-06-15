use super::{
    project_point_to_certified_plane_2d_digest,
    project_point_to_certified_plane_2d_identity_entries, ProjectPointToCertifiedPlane2DBasis,
    ProjectPointToCertifiedPlane2DMutationEvidence,
    ProjectPointToCertifiedPlane2DPerformanceCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DReceipt {
    basis: ProjectPointToCertifiedPlane2DBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    mutation_evidence: ProjectPointToCertifiedPlane2DMutationEvidence,
    counters: ProjectPointToCertifiedPlane2DPerformanceCounters,
}

impl ProjectPointToCertifiedPlane2DReceipt {
    pub(crate) fn new(
        basis: ProjectPointToCertifiedPlane2DBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        mutation_evidence: ProjectPointToCertifiedPlane2DMutationEvidence,
        counters: ProjectPointToCertifiedPlane2DPerformanceCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            fact_digest,
            mutation_evidence,
            counters,
        }
    }

    pub(crate) fn digest_parts(
        basis: &ProjectPointToCertifiedPlane2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = project_point_to_certified_plane_2d_identity_entries(basis)
            .into_iter()
            .map(|entry| format!("{}:{}", entry.locus(), entry.value()))
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &ProjectPointToCertifiedPlane2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        project_point_to_certified_plane_2d_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub fn basis(&self) -> &ProjectPointToCertifiedPlane2DBasis {
        &self.basis
    }

    pub fn source_point_identity(&self) -> &str {
        self.basis.source_point_identity()
    }

    pub fn point_2d(&self) -> [f64; 2] {
        self.basis.point_2d()
    }

    pub fn signed_distance_to_plane_bits(&self) -> u64 {
        self.basis.signed_distance_to_plane_bits()
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        self.basis.local_frame_fact_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn mutation_evidence(&self) -> &ProjectPointToCertifiedPlane2DMutationEvidence {
        &self.mutation_evidence
    }

    pub fn counters(&self) -> ProjectPointToCertifiedPlane2DPerformanceCounters {
        self.counters
    }
}
