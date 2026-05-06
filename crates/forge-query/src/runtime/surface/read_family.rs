use crate::identity::hash_parts;

use super::{ForgeQueryReadDomainInvariantSummary, ForgeQueryReadGraph};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadFamilyAdmission {
    KernelOnly,
    DomainInvariantAdmitted(ForgeQueryReadFamilyInvariantEvidence),
}

impl ForgeQueryReadFamilyAdmission {
    fn digest_component(&self) -> String {
        match self {
            Self::KernelOnly => "admission:kernel_only".to_string(),
            Self::DomainInvariantAdmitted(evidence) => {
                format!("admission:{}", evidence.evidence_digest())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadFamilyInvariantEvidence {
    invariant_family: String,
    domain_invariant_summary: ForgeQueryReadDomainInvariantSummary,
    evidence_digest: String,
}

impl ForgeQueryReadFamilyInvariantEvidence {
    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn domain_invariant_summary(&self) -> &ForgeQueryReadDomainInvariantSummary {
        &self.domain_invariant_summary
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub(in crate::runtime) fn new(
        invariant_family: impl Into<String>,
        domain_invariant_summary: ForgeQueryReadDomainInvariantSummary,
    ) -> Self {
        let invariant_family = invariant_family.into();
        let evidence_digest = hash_parts(&[
            "forge_query_read_family_invariant_evidence_v1".to_string(),
            format!("family:{invariant_family}"),
            format!("summary:{}", domain_invariant_summary.summary_digest()),
        ]);
        Self {
            invariant_family,
            domain_invariant_summary,
            evidence_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadFamily {
    family_name: String,
    family_digest: String,
    admission: ForgeQueryReadFamilyAdmission,
    read_graph: ForgeQueryReadGraph,
}

impl ForgeQueryReadFamily {
    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub fn admission(&self) -> &ForgeQueryReadFamilyAdmission {
        &self.admission
    }

    pub fn read_graph(&self) -> &ForgeQueryReadGraph {
        &self.read_graph
    }

    pub(in crate::runtime) fn new_kernel_only(
        family_name: impl Into<String>,
        read_graph: ForgeQueryReadGraph,
    ) -> Self {
        Self::new(
            family_name,
            ForgeQueryReadFamilyAdmission::KernelOnly,
            read_graph,
        )
    }

    pub(in crate::runtime) fn new_domain_invariant_admitted(
        family_name: impl Into<String>,
        evidence: ForgeQueryReadFamilyInvariantEvidence,
        read_graph: ForgeQueryReadGraph,
    ) -> Self {
        Self::new(
            family_name,
            ForgeQueryReadFamilyAdmission::DomainInvariantAdmitted(evidence),
            read_graph,
        )
    }

    fn new(
        family_name: impl Into<String>,
        admission: ForgeQueryReadFamilyAdmission,
        read_graph: ForgeQueryReadGraph,
    ) -> Self {
        let family_name = family_name.into();
        let family_digest = hash_parts(&[
            "forge_query_read_family_v1".to_string(),
            format!("family_name:{family_name}"),
            admission.digest_component(),
            format!("read_graph:{}", read_graph.digest()),
        ]);
        Self {
            family_name,
            family_digest,
            admission,
            read_graph,
        }
    }
}
