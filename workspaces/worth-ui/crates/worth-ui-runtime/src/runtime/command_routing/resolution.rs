pub(super) fn prefix_owners(
    candidates: &[&super::candidate::UiCommandRouteCandidate],
) -> Box<[Option<crate::capability::UiCommandRegistrationOwner>]> {
    candidates
        .iter()
        .map(|candidate| candidate.route().registration_owner())
        .collect()
}

pub(super) fn eligible_candidates<'a>(
    candidates: Vec<&'a super::candidate::UiCommandRouteCandidate>,
    repeat: bool,
    context: &super::UiCommandRoutingContext,
    policy: crate::declaration::UiCommandRoutingPolicy,
) -> (
    Vec<&'a super::candidate::UiCommandRouteCandidate>,
    Option<super::UiCommandRoutingSuppression>,
) {
    let mut suppression = None;
    let eligible = candidates
        .into_iter()
        .filter(|candidate| context.scope_is_active(candidate.route()))
        .filter(|candidate| context.supports_consumption(candidate.route().context()))
        .filter(|candidate| {
            let route = candidate.route();
            let denied = if policy.suppresses_repeats()
                && repeat
                && route.repeat_policy() == crate::capability::UiCommandRepeatPolicy::Suppress
            {
                Some(super::UiCommandRoutingSuppression::RepeatSuppressed)
            } else if policy.suppresses_during_ime()
                && context.ime_composing()
                && route.text_input_policy() != crate::capability::UiCommandTextInputPolicy::Allow
            {
                Some(super::UiCommandRoutingSuppression::ImeComposition)
            } else if policy.suppresses_during_text_input()
                && context.text_entry_active()
                && route.text_input_policy()
                    == crate::capability::UiCommandTextInputPolicy::SuppressDuringCompositionAndTextInput
            {
                Some(super::UiCommandRoutingSuppression::TextEntry)
            } else {
                None
            };
            if denied.is_some() {
                suppression = suppression.or(denied);
                false
            } else {
                true
            }
        })
        .collect();
    (eligible, suppression)
}

pub(super) fn maximum_rank(
    candidates: &[&super::candidate::UiCommandRouteCandidate],
) -> (u8, i16, u32) {
    candidates
        .iter()
        .map(|candidate| candidate.rank())
        .max()
        .expect("non-empty candidate set has a maximum rank")
}

pub(super) fn resolve_complete(
    candidates: Vec<&super::candidate::UiCommandRouteCandidate>,
    application: &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    context: &super::UiCommandRoutingContext,
) -> super::UiCommandRoutingOutcome {
    let maximum = maximum_rank(&candidates);
    let winners = candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.rank() == maximum)
        .collect::<Vec<_>>();
    if winners.len() != 1 {
        return super::UiCommandRoutingOutcome::Ambiguous(super::UiCommandAmbiguity::new(
            winners
                .into_iter()
                .map(|candidate| candidate.command().clone())
                .collect(),
        ));
    }
    let winner = winners[0];
    let losers = candidates
        .into_iter()
        .filter(|candidate| candidate.command() != winner.command())
        .take(16)
        .map(|candidate| {
            let reason =
                if candidate.route().scope().precedence() < winner.route().scope().precedence() {
                    super::UiCommandRouteLossReason::LowerScopePrecedence
                } else if candidate.route().priority() < winner.route().priority() {
                    super::UiCommandRouteLossReason::LowerDeclaredPriority
                } else {
                    super::UiCommandRouteLossReason::LowerSpecificity
                };
            super::UiCommandRouteLoss::new(candidate.command().clone(), reason)
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    super::UiCommandRoutingOutcome::Routed(super::UiCommandRouteReceipt::new(
        winner,
        application,
        context,
        super::UiCommandInvocationOrigin::KeyboardShortcut,
        losers,
    ))
}
