use super::{
    UiIdentityLifecycleDecision, UiIdentityLifecycleDenial, UiIdentityLifecycleEntry,
    UiResolvedIdentityLifecycle,
};

pub(crate) struct UiIdentityLifecycleResolver;

pub(crate) struct UiIdentityLifecycleRecoveryStop {
    denial: UiIdentityLifecycleDenial,
    scope: Box<super::super::UiResolvedAffectedScope>,
}

impl UiIdentityLifecycleResolver {
    pub(crate) fn resolve(
        scope: super::super::UiResolvedAffectedScope,
    ) -> Result<UiResolvedIdentityLifecycle, UiIdentityLifecycleDenial> {
        Self::resolve_recoverable(scope).map_err(UiIdentityLifecycleRecoveryStop::into_denial)
    }

    pub(crate) fn resolve_recoverable(
        scope: super::super::UiResolvedAffectedScope,
    ) -> Result<UiResolvedIdentityLifecycle, UiIdentityLifecycleRecoveryStop> {
        let selected = match select_lifecycle(&scope) {
            Ok(selected) => selected,
            Err(denial) => {
                return Err(UiIdentityLifecycleRecoveryStop {
                    denial,
                    scope: Box::new(scope),
                })
            }
        };
        Ok(UiResolvedIdentityLifecycle::new(scope, selected))
    }
}

fn select_lifecycle(
    scope: &super::super::UiResolvedAffectedScope,
) -> Result<Box<[UiIdentityLifecycleEntry]>, UiIdentityLifecycleDenial> {
    Ok(scope
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
        .into_boxed_slice())
}

impl UiIdentityLifecycleRecoveryStop {
    pub(crate) fn into_denial(self) -> UiIdentityLifecycleDenial {
        self.denial
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiIdentityLifecycleDenial,
        super::super::UiResolvedAffectedScope,
    ) {
        (self.denial, *self.scope)
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
