use worth_query::facade::runtime::{
    WorthQueryCheckedDomainCapabilityOutcome, WorthQueryDomainCapabilityTargetKind,
};

fn main() {
    let _ = WorthQueryCheckedDomainCapabilityOutcome::<()>
    {
        category: "support-traceability",
        target_kind: WorthQueryDomainCapabilityTargetKind::IntentDeclaration,
        semantic_posture: "declaration-support",
        inner: todo!(),
    };
}
