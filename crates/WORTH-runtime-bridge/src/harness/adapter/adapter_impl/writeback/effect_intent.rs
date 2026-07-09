use super::*;

pub(in crate::harness::adapter::adapter_impl::writeback) fn writeback_effect_intent(
    effect_class: BridgeWritebackEffectClass,
    marker: impl Into<String>,
) -> BridgeWritebackEffectIntent {
    let aspect_key = match effect_class {
        BridgeWritebackEffectClass::ProjectedStateDiff => "bridge.writeback.projected-state-diff",
        BridgeWritebackEffectClass::AspectReconciliation => {
            "bridge.writeback.aspect-reconciliation"
        }
    };
    BridgeWritebackEffectIntent::validated_scalar_patch(
        effect_class,
        AspectKey::new(aspect_key).expect("static writeback effect aspect key is valid"),
        AspectValue::String(marker.into().into()),
    )
    .expect("writeback harness effect intent should validate as a foundational scalar patch")
}
