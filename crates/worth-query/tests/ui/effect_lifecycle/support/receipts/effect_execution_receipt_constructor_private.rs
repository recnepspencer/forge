#![allow(unreachable_code)]

use worth_query::facade::{
    EffectAuthorityLane, EffectExecutionReceipt, EffectFamily, EffectReceiptArtifactKind,
};

fn main() {
    let _ = EffectExecutionReceipt {
        artifact: todo!(),
        receipt_family: EffectReceiptArtifactKind::WorthQueryIntentExecution,
        declared_effect_family: EffectFamily::Mutation,
        authority_lane: EffectAuthorityLane::Relational,
        basis_family: worth_query::facade::BasisFamily::CurrentHead,
        receipt_digest: String::new(),
        decision_trace: todo!(),
        integrity_markers: todo!(),
        counters: todo!(),
    };
}
