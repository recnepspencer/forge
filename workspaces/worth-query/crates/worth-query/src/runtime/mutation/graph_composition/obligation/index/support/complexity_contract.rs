use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationIndexComplexityContractStatus {
    Verified,
}

impl WorthQueryGraphObligationIndexComplexityContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationIndexComplexityContract {
    name: &'static str,
    status: WorthQueryGraphObligationIndexComplexityContractStatus,
    complexity_bound: &'static str,
    counter_basis: &'static str,
    contract_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationIndexComplexityContract {
    pub(super) fn new(
        name: &'static str,
        complexity_bound: &'static str,
        counter_basis: &'static str,
    ) -> Self {
        let status = WorthQueryGraphObligationIndexComplexityContractStatus::Verified;
        let contract_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationIndexComplexityContract,
        )
        .field_shape(WorthQueryEvidenceTag::new("name"), name)
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("complexity_bound"),
            complexity_bound,
        )
        .field_shape(WorthQueryEvidenceTag::new("counter_basis"), counter_basis)
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

    pub fn status(&self) -> WorthQueryGraphObligationIndexComplexityContractStatus {
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

    pub(crate) fn contract_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.contract_digest
    }
}

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn graph_obligation_index_complexity_contracts(
) -> Vec<WorthQueryGraphObligationIndexComplexityContract> {
    vec![
        WorthQueryGraphObligationIndexComplexityContract::new(
            "graph-obligation-index-build",
            "O(registered_obligations log registered_obligations)",
            "registration_count + bucket_count",
        ),
        WorthQueryGraphObligationIndexComplexityContract::new(
            "graph-obligation-dispatch-selection",
            "O(touch_lookup_keys * operating_world_lookup_keys + matched_obligations log matched_obligations)",
            "attempted_bucket_lookup_count + candidate_registration_count + registration_full_scan_count",
        ),
    ]
}
