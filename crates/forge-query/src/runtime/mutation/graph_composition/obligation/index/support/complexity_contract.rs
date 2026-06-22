use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationIndexComplexityContractStatus {
    Verified,
}

impl ForgeQueryGraphObligationIndexComplexityContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationIndexComplexityContract {
    name: &'static str,
    status: ForgeQueryGraphObligationIndexComplexityContractStatus,
    complexity_bound: &'static str,
    counter_basis: &'static str,
    contract_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationIndexComplexityContract {
    pub(super) fn new(
        name: &'static str,
        complexity_bound: &'static str,
        counter_basis: &'static str,
    ) -> Self {
        let status = ForgeQueryGraphObligationIndexComplexityContractStatus::Verified;
        let contract_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationIndexComplexityContract,
        )
        .field_shape(ForgeQueryEvidenceTag::new("name"), name)
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("complexity_bound"),
            complexity_bound,
        )
        .field_shape(ForgeQueryEvidenceTag::new("counter_basis"), counter_basis)
        .seal();
        Self {
            name,
            status,
            complexity_bound,
            counter_basis,
            contract_digest,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn status(&self) -> ForgeQueryGraphObligationIndexComplexityContractStatus {
        self.status
    }

    pub fn complexity_bound(&self) -> &'static str {
        self.complexity_bound
    }

    pub fn counter_basis(&self) -> &'static str {
        self.counter_basis
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_str()
    }

    pub(crate) fn contract_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.contract_digest
    }
}

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn graph_obligation_index_complexity_contracts(
) -> Vec<ForgeQueryGraphObligationIndexComplexityContract> {
    vec![
        ForgeQueryGraphObligationIndexComplexityContract::new(
            "graph-obligation-index-build",
            "O(registered_obligations log registered_obligations)",
            "registration_count + bucket_count",
        ),
        ForgeQueryGraphObligationIndexComplexityContract::new(
            "graph-obligation-dispatch-selection",
            "O(touch_lookup_keys * operating_world_lookup_keys + matched_obligations log matched_obligations)",
            "attempted_bucket_lookup_count + candidate_registration_count + registration_full_scan_count",
        ),
    ]
}
