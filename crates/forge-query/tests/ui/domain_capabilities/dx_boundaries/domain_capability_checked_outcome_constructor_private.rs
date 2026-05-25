use forge_query::facade::runtime::{
    ForgeQueryCheckedDomainCapabilityOutcome, ForgeQueryDomainCapabilityTargetKind,
};

fn main() {
    let _ = ForgeQueryCheckedDomainCapabilityOutcome::<()>
    {
        category: "support-traceability",
        target_kind: ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
        semantic_posture: "declaration-support",
        inner: todo!(),
    };
}
