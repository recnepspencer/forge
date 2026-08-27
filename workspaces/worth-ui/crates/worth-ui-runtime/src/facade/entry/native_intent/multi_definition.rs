use super::{
    stopped, WorthUiNativeIntentIngress, WorthUiNativeIntentStop, WorthUiNativeIntentTransition,
};

impl super::super::WorthUiNativeApplicationShell {
    pub fn admit_native_intent_progress_triplet<I1, D1, I2, D2, I3, D3>(
        &mut self,
        first: crate::facade::intent::UiIntentDefinition<I1, D1>,
        second: crate::facade::intent::UiIntentDefinition<I2, D2>,
        third: crate::facade::intent::UiIntentDefinition<I3, D3>,
        progress: crate::native_platform::UiNativeApplicationObservationProgress,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentIngress
    where
        I1: crate::facade::intent::UiIntent,
        D1: crate::facade::intent::UiIntentDefinitionDestination,
        I2: crate::facade::intent::UiIntent,
        D2: crate::facade::intent::UiIntentDefinitionDestination,
        I3: crate::facade::intent::UiIntent,
        D3: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let outcomes = progress.into_settlement().into_outcomes().into_vec();
        let mut transitions = Vec::new();
        let mut dismissals = Vec::new();
        let mut duplicate_batches = 0;
        let mut interaction_stops = Vec::new();
        for outcome in outcomes {
            match outcome {
                crate::facade::interaction::UiHostInteractionIngressOutcome::Applied(receipt) => {
                    for transition in receipt.into_transitions() {
                        match transition {
                            crate::facade::interaction::UiInteractionTransition::Semantic(
                                interaction,
                            ) => transitions.push(self.admit_triplet_semantic(
                                first,
                                second,
                                third,
                                interaction,
                                deadline,
                            )),
                            crate::facade::interaction::UiInteractionTransition::DismissRequested(
                                dismissal,
                            ) => dismissals.push(dismissal),
                            _ => {}
                        }
                    }
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Duplicate(_) => {
                    duplicate_batches += 1;
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Quarantined(stop) => {
                    interaction_stops.push(
                        super::WorthUiNativeInteractionIngressStop::Quarantined(stop),
                    );
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Denied(stop) => {
                    interaction_stops
                        .push(super::WorthUiNativeInteractionIngressStop::Denied(stop));
                }
            }
        }
        WorthUiNativeIntentIngress {
            transitions: transitions.into_boxed_slice(),
            dismissals: dismissals.into_boxed_slice(),
            duplicate_batches,
            interaction_stops: interaction_stops.into_boxed_slice(),
        }
    }

    fn admit_triplet_semantic<I1, D1, I2, D2, I3, D3>(
        &mut self,
        first: crate::facade::intent::UiIntentDefinition<I1, D1>,
        second: crate::facade::intent::UiIntentDefinition<I2, D2>,
        third: crate::facade::intent::UiIntentDefinition<I3, D3>,
        interaction: crate::facade::interaction::UiSemanticInteraction,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentTransition
    where
        I1: crate::facade::intent::UiIntent,
        D1: crate::facade::intent::UiIntentDefinitionDestination,
        I2: crate::facade::intent::UiIntent,
        D2: crate::facade::intent::UiIntentDefinitionDestination,
        I3: crate::facade::intent::UiIntent,
        D3: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let route = match self.session.resolve_intent_route(
            crate::facade::intent::UiIntentRouteSource::mounted_interaction(interaction),
        ) {
            Ok(route) => route,
            Err(stop) => return stopped(WorthUiNativeIntentStop::Route(stop), None),
        };
        let definition = match &route {
            crate::facade::intent::UiIntentRouteResolution::Product(route) => route.definition_id(),
            crate::facade::intent::UiIntentRouteResolution::Confirmation(route) => {
                route.definition_id()
            }
        };
        if definition == first.id() {
            self.admit_native_resolved_intent(first, route, deadline)
        } else if definition == second.id() {
            self.admit_native_resolved_intent(second, route, deadline)
        } else if definition == third.id() {
            self.admit_native_resolved_intent(third, route, deadline)
        } else {
            stopped(WorthUiNativeIntentStop::DefinitionNotSelected, None)
        }
    }

    pub fn admit_native_intent_progress_pair<I1, D1, I2, D2>(
        &mut self,
        first: crate::facade::intent::UiIntentDefinition<I1, D1>,
        second: crate::facade::intent::UiIntentDefinition<I2, D2>,
        progress: crate::native_platform::UiNativeApplicationObservationProgress,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentIngress
    where
        I1: crate::facade::intent::UiIntent,
        D1: crate::facade::intent::UiIntentDefinitionDestination,
        I2: crate::facade::intent::UiIntent,
        D2: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let outcomes = progress.into_settlement().into_outcomes().into_vec();
        let mut transitions = Vec::new();
        let mut dismissals = Vec::new();
        let mut duplicate_batches = 0;
        let mut interaction_stops = Vec::new();
        for outcome in outcomes {
            match outcome {
                crate::facade::interaction::UiHostInteractionIngressOutcome::Applied(receipt) => {
                    for transition in receipt.into_transitions() {
                        match transition {
                            crate::facade::interaction::UiInteractionTransition::Semantic(
                                interaction,
                            ) => transitions.push(self.admit_pair_semantic(
                                first,
                                second,
                                interaction,
                                deadline,
                            )),
                            crate::facade::interaction::UiInteractionTransition::DismissRequested(
                                dismissal,
                            ) => dismissals.push(dismissal),
                            _ => {}
                        }
                    }
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Duplicate(_) => {
                    duplicate_batches += 1;
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Quarantined(stop) => {
                    interaction_stops.push(
                        super::WorthUiNativeInteractionIngressStop::Quarantined(stop),
                    );
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Denied(stop) => {
                    interaction_stops
                        .push(super::WorthUiNativeInteractionIngressStop::Denied(stop));
                }
            }
        }
        WorthUiNativeIntentIngress {
            transitions: transitions.into_boxed_slice(),
            dismissals: dismissals.into_boxed_slice(),
            duplicate_batches,
            interaction_stops: interaction_stops.into_boxed_slice(),
        }
    }

    fn admit_pair_semantic<I1, D1, I2, D2>(
        &mut self,
        first: crate::facade::intent::UiIntentDefinition<I1, D1>,
        second: crate::facade::intent::UiIntentDefinition<I2, D2>,
        interaction: crate::facade::interaction::UiSemanticInteraction,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentTransition
    where
        I1: crate::facade::intent::UiIntent,
        D1: crate::facade::intent::UiIntentDefinitionDestination,
        I2: crate::facade::intent::UiIntent,
        D2: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let route = match self.session.resolve_intent_route(
            crate::facade::intent::UiIntentRouteSource::mounted_interaction(interaction),
        ) {
            Ok(route) => route,
            Err(stop) => return stopped(WorthUiNativeIntentStop::Route(stop), None),
        };
        let definition = match &route {
            crate::facade::intent::UiIntentRouteResolution::Product(route) => route.definition_id(),
            crate::facade::intent::UiIntentRouteResolution::Confirmation(route) => {
                route.definition_id()
            }
        };
        if definition == first.id() {
            self.admit_native_resolved_intent(first, route, deadline)
        } else if definition == second.id() {
            self.admit_native_resolved_intent(second, route, deadline)
        } else {
            stopped(WorthUiNativeIntentStop::DefinitionNotSelected, None)
        }
    }
}
