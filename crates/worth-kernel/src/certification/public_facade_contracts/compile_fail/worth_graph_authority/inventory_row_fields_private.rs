use worth_kernel::query_graph_authority_gate::{
    WorthGraphAuthorityAction, WorthGraphAuthorityDeletionTarget,
    WorthGraphAuthorityDiscoverySource, WorthGraphAuthorityInventoryRow,
    WorthGraphAuthorityOwner, WorthGraphAuthorityRowClass, WorthGraphAuthoritySourceScope,
};

fn main() {
    let _ = WorthGraphAuthorityInventoryRow {
        source_id: "forged",
        source_path: "crates/worth-kernel/src/forged.rs",
        source_scope: WorthGraphAuthoritySourceScope::ExactSource,
        owner: WorthGraphAuthorityOwner::Kernel,
        row_class: WorthGraphAuthorityRowClass::RootAuthority,
        deletion_target: WorthGraphAuthorityDeletionTarget::None,
        discovery_source: WorthGraphAuthorityDiscoverySource::SearchSeed,
        action: WorthGraphAuthorityAction::Keep,
        authority_claim: "forged claim",
        replacement_or_blocker: "forged replacement",
        qa_evidence: "forged evidence",
    };
}
