#![allow(unreachable_code)]

use worth_query::facade::{
    EffectReceiptArtifactKind, EffectReceiptTransitionRule, EffectReceiptTransitionRules,
};

fn main() {
    let _ = EffectReceiptTransitionRules {
        receipt_family: EffectReceiptArtifactKind::WorthQueryIntentExecution,
        rules: vec![todo!() as EffectReceiptTransitionRule],
        rules_digest: String::new(),
    };
}
