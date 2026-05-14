#![allow(unreachable_code)]

use forge_query::facade::{
    EffectReceiptArtifactKind, EffectReceiptTransitionRule, EffectReceiptTransitionRules,
};

fn main() {
    let _ = EffectReceiptTransitionRules {
        receipt_family: EffectReceiptArtifactKind::ForgeQueryIntentExecution,
        rules: vec![todo!() as EffectReceiptTransitionRule],
        rules_digest: String::new(),
    };
}
