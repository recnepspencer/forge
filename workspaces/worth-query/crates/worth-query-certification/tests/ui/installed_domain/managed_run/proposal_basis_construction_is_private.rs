use worth_query_execution::facade::provider_session::{
    WorthQueryProvisionalProposalBasis, WorthQueryProvisionalProposalBasisParts,
};

fn main() {
    let _ = WorthQueryProvisionalProposalBasis::new(WorthQueryProvisionalProposalBasisParts {
        source_occurrence: "source".into(),
        search_occurrence: "search".into(),
        candidate_identity: "candidate".into(),
        transformation_evidence: "transformation".into(),
        semantic_basis_identity: "basis".into(),
        target_generation: 1,
        installed_policy_identity: "policy".into(),
        correspondence_identity: "correspondence".into(),
        identity_consequence_identity: "consequence".into(),
    });
}
