use worth_query::facade::foundation::{WorthQueryBindingLinkedArtifacts, WorthQueryDeclarationEntryOrchestrationStage, WorthQueryOrdinaryBindingCheckedTopologyKind, WorthQueryOrdinaryCheckedTopology};

fn main() {
    let linked = WorthQueryBindingLinkedArtifacts::new();
    let _ = WorthQueryOrdinaryCheckedTopology::binding(
        WorthQueryOrdinaryBindingCheckedTopologyKind::WrongWorld,
        linked,
    );
    let _ = WorthQueryOrdinaryCheckedTopology::orchestration(
        WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        Some(String::from("digest")),
        None,
    );
}
