use crate::runtime::WorthUiLiveViewEffectIntentGraphPosture;

pub(crate) fn live_view_effect_intent_graph_posture(
    effect: &str,
) -> WorthUiLiveViewEffectIntentGraphPosture {
    if effect_authority_namespace(effect) == Some("validation.effect") {
        WorthUiLiveViewEffectIntentGraphPosture::Supported
    } else {
        WorthUiLiveViewEffectIntentGraphPosture::Unsupported
    }
}

fn effect_authority_namespace(effect: &str) -> Option<&str> {
    let mut parts = effect.split('.');
    let authority = parts.next()?;
    let family = parts.next()?;
    if authority.is_empty() || family.is_empty() || parts.next().is_none() {
        return None;
    }
    Some(effect.split_at(authority.len() + family.len() + 1).0)
}
