#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphReadAccessRebuildBasis {
    AuthoritativeRelationTruth,
    AuthoritativeFieldTruth,
    ReadGraphProof,
    OperationResolutionProof,
    SelectivityProof,
    RuntimeSupportRequired,
}

impl WorthQueryGraphReadAccessRebuildBasis {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeRelationTruth => "authoritative_relation_truth",
            Self::AuthoritativeFieldTruth => "authoritative_field_truth",
            Self::ReadGraphProof => "read_graph_proof",
            Self::OperationResolutionProof => "operation_resolution_proof",
            Self::SelectivityProof => "selectivity_proof",
            Self::RuntimeSupportRequired => "runtime_support_required",
        }
    }
}
