use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use std::collections::BTreeMap;

impl ResourceRuntimeState {
    pub fn admit_resource_completion_batch(
        &mut self,
        completions: impl IntoIterator<Item = RawCompletionEnvelope>,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceCompletionBatchAdmissionReport {
        let mut completions = completions.into_iter().collect::<Vec<_>>();
        let input_width = completions.len() as u32;
        let density_strategy = ResourceDensityStrategy::completion_batch(
            input_width,
            self.in_flight_by_request.len() as u32,
        );
        completions.sort();

        let mut admitted_completions = Vec::new();
        let mut denied_completions = Vec::new();
        let mut seen_identities = BTreeMap::<
            (
                ResourceRequestId,
                ResourceGeneration,
                ResourceBranchEpoch,
                ResourceAttemptId,
            ),
            RawCompletionEnvelope,
        >::new();
        let mut duplicate_width = 0_u32;

        for raw in completions {
            let identity = (
                raw.request_id(),
                raw.generation(),
                raw.branch_epoch(),
                raw.attempt(),
            );
            if let Some(prior) = seen_identities.get(&identity) {
                duplicate_width = duplicate_width.saturating_add(1);
                if let Some(telemetry) = telemetry.as_deref_mut() {
                    telemetry.resource_completion_validation_count += 1;
                }
                let class = if prior == &raw {
                    CompletionDenialClass::Duplicate
                } else {
                    CompletionDenialClass::Contradictory
                };
                let denied_node = self
                    .in_flight_by_request
                    .get(&raw.request_id())
                    .map(|in_flight| in_flight.node())
                    .or_else(|| {
                        self.retained_in_flight_history_by_request
                            .get(&raw.request_id())
                            .map(|retained| retained.node())
                    });
                let denied = self
                    .deny_completion(&raw, class, denied_node, telemetry.as_deref_mut(), false)
                    .denied_completion()
                    .expect("batch duplicate denial should retain denied completion");
                denied_completions.push(denied);
                continue;
            }
            seen_identities.insert(identity, raw.clone());

            let report =
                self.admit_resource_completion_with_boundary(raw, telemetry.as_deref_mut(), false);
            if let Some(admitted) = report.admitted_completion() {
                admitted_completions.push(admitted);
            }
            if let Some(denied) = report.denied_completion() {
                denied_completions.push(denied);
            }
        }

        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_completion_batch_admission_count += 1;
        }
        let admitted_count = admitted_completions.len() as u32;
        let denied_count = denied_completions.len() as u32;
        let envelope = ResourceBoundaryPerformanceEnvelope::completion_batch_admission(
            input_width,
            admitted_count,
            denied_count,
        )
        .with_density_strategy(density_strategy);
        let performance = telemetry
            .as_deref_mut()
            .map(|telemetry| Self::record_boundary_performance(telemetry, envelope))
            .unwrap_or(envelope);
        ResourceCompletionBatchAdmissionReport::new(
            admitted_completions,
            denied_completions,
            input_width,
            duplicate_width,
            performance,
        )
    }
}
