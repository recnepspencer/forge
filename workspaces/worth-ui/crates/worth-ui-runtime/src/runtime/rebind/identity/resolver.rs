use super::{
    UiIdentityLifecycleDecision, UiIdentityLifecycleDenial, UiIdentityLifecycleEntry,
    UiResolvedIdentityLifecycle,
};

pub(crate) struct UiIdentityLifecycleResolver;

impl UiIdentityLifecycleResolver {
    pub(crate) fn resolve(
        scope: super::super::UiResolvedAffectedScope,
    ) -> Result<UiResolvedIdentityLifecycle, UiIdentityLifecycleDenial> {
        let selected = scope
            .consumers()
            .iter()
            .map(|consumer| {
                let decision = match scope
                    .source_succession()
                    .and_then(|succession| succession.identity_lifecycle_index())
                {
                    Some(index) => index.selected_decision(
                        consumer.key(),
                        consumer.predecessor(),
                        consumer.candidate(),
                    )?,
                    None => decision_from_presence(
                        consumer.key(),
                        consumer.predecessor(),
                        consumer.candidate(),
                    )?,
                };
                Ok(UiIdentityLifecycleEntry::new(
                    consumer.key().clone(),
                    consumer.predecessor(),
                    consumer.candidate(),
                    decision,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        Ok(UiResolvedIdentityLifecycle::new(scope, selected))
    }
}

fn decision_from_presence(
    key: &crate::graph::UiGraphFactConsumerKey,
    predecessor: Option<crate::graph::UiGraphFactConsumerIdentity>,
    candidate: Option<crate::graph::UiGraphFactConsumerIdentity>,
) -> Result<UiIdentityLifecycleDecision, UiIdentityLifecycleDenial> {
    match (predecessor, candidate) {
        (Some(_), Some(_)) => Ok(UiIdentityLifecycleDecision::Preserve),
        (None, Some(_)) => Ok(UiIdentityLifecycleDecision::Create),
        (Some(_), None) => Ok(UiIdentityLifecycleDecision::Retire),
        (None, None) => {
            Err(UiIdentityLifecycleDenial::MissingSelectedConsumer { key: key.clone() })
        }
    }
}
