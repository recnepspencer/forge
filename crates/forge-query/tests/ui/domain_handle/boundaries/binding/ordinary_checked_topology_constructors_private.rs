use forge_query::facade::{
    ForgeQueryBindingLinkedArtifacts, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryCheckedTopology,
};

fn main() {
    let linked = ForgeQueryBindingLinkedArtifacts::new();
    let _ = ForgeQueryOrdinaryCheckedTopology::binding(
        ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongWorld,
        linked,
    );
    let _ = ForgeQueryOrdinaryCheckedTopology::orchestration(
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        Some(String::from("digest")),
        None,
    );
}
