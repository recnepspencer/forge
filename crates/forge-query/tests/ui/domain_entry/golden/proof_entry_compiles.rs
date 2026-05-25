use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::WorkflowOrchestration];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleProofDomain;

impl ForgeQueryDomainEntryMarker for ExampleProofDomain {
    fn domain_key(&self) -> &'static str {
        "example.proof"
    }

    fn display_name(&self) -> &'static str {
        "ExampleProofDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let proof = query.domain_proof_root(ExampleProofDomain);

    let _ = proof.domain_key();
    let _ = proof.display_name();
}
