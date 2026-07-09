use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::WorkflowOrchestration];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleProofDomain;

impl WorthQueryDomainEntryMarker for ExampleProofDomain {
    fn domain_key(&self) -> &'static str {
        "example.proof"
    }

    fn display_name(&self) -> &'static str {
        "ExampleProofDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let proof = query.domain_proof_root(ExampleProofDomain);

    let _ = proof.domain_key();
    let _ = proof.display_name();
}
