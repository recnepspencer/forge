pub(in crate::domain_computation::primary_graph::conditional_operation) fn retire_stale_retained_wakes<
    Clock,
    Input,
>(
    wakes: &mut Vec<
        crate::domain_computation::primary_graph::conditional_operation::signal_decision_reentry::WorthQueryRetainedConditionalWake,
    >,
    intents: &std::collections::BTreeMap<
        String,
        crate::domain_computation::primary_graph::conditional_operation::temporal_reconstruction::WorthQueryReconstructedTemporalIntent<Clock, Input>,
    >,
) {
    wakes.retain(|wake| {
        intents
            .get(wake.due.intent_identity().as_str())
            .is_some_and(|intent| {
                let candidate = intent.candidate();
                wake.due.revision() == candidate.revision()
                    && wake.due.due_coordinate() == candidate.due().nanoseconds()
                    && wake.due.idempotency_identity() == candidate.idempotency().as_str()
            })
    });
}
