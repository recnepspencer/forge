#![allow(unreachable_code)]

use forge_query::facade::{
    EffectAuthorityLane, EffectExecutionReceipt, EffectFamily, EffectReceiptArtifactKind,
};

fn main() {
    let _ = EffectExecutionReceipt {
        artifact: todo!(),
        receipt_family: EffectReceiptArtifactKind::ForgeQueryIntentExecution,
        declared_effect_family: EffectFamily::Mutation,
        authority_lane: EffectAuthorityLane::Relational,
        basis_family: forge_query::facade::BasisFamily::CurrentHead,
        receipt_digest: String::new(),
        decision_trace: todo!(),
        integrity_markers: todo!(),
        counters: todo!(),
    };
}
