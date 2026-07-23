use worth_ui_host_contract::{
    WorthUiHostOutputDisposition, WorthUiHostOutputEnvelope, WorthUiHostOutputGeneration,
    WorthUiHostOutputGenerationDenial, WorthUiOperationalHostAdapter,
};

pub(super) fn consume_active_host_output(
    adapter: &dyn WorthUiOperationalHostAdapter,
    expected_generation: WorthUiHostOutputGeneration,
    output: &WorthUiHostOutputEnvelope,
) -> WorthUiHostOutputDisposition {
    admit_and_consume_active_host_output(adapter, expected_generation, output)
        .expect("runtime-minted host output must retain its exact active generation")
}

fn admit_and_consume_active_host_output(
    adapter: &dyn WorthUiOperationalHostAdapter,
    expected_generation: WorthUiHostOutputGeneration,
    output: &WorthUiHostOutputEnvelope,
) -> Result<WorthUiHostOutputDisposition, WorthUiHostOutputGenerationDenial> {
    output.validate_generation(expected_generation)?;
    Ok(adapter.consume_output(output))
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use worth_ui_host_contract::{
        UiHostObservationValue, UiMeasurementRequest, WorthUiHeadlessHost,
        WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiHostOutputGenerationDenialReason,
        WorthUiMeasurementHostAdapter, WorthUiOperationalHostAdapter, WorthUiOrdinaryHostOutput,
        WorthUiOrdinaryHostOutputTarget,
    };

    use super::*;

    struct CountingAdapter {
        native_call_count: Cell<u32>,
    }

    impl WorthUiMeasurementHostAdapter for CountingAdapter {
        fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
            WorthUiHeadlessHost.observe_measurement(request)
        }
    }

    impl WorthUiOperationalHostAdapter for CountingAdapter {
        fn operational_host_contract(&self) -> WorthUiHostContract {
            WorthUiHeadlessHost.operational_host_contract()
        }

        fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
            WorthUiHeadlessHost.operational_capability_report()
        }

        fn consume_output(
            &self,
            _output: &WorthUiHostOutputEnvelope,
        ) -> WorthUiHostOutputDisposition {
            self.native_call_count
                .set(self.native_call_count.get().saturating_add(1));
            WorthUiHostOutputDisposition::Consumed
        }
    }

    #[test]
    fn stale_or_foreign_generation_denies_before_adapter_work() {
        let expected = WorthUiHostOutputGeneration::new(7, 11, 13, 17);
        let cases = [
            (
                WorthUiHostOutputGeneration::new(8, 11, 13, 17),
                WorthUiHostOutputGenerationDenialReason::HostSessionMismatch,
            ),
            (
                WorthUiHostOutputGeneration::new(7, 12, 13, 17),
                WorthUiHostOutputGenerationDenialReason::ActiveArtifactMismatch,
            ),
            (
                WorthUiHostOutputGeneration::new(7, 11, 14, 17),
                WorthUiHostOutputGenerationDenialReason::ActivePlanMismatch,
            ),
            (
                WorthUiHostOutputGeneration::new(7, 11, 13, 18),
                WorthUiHostOutputGenerationDenialReason::FrameEpochMismatch,
            ),
        ];

        for (generation, expected_reason) in cases {
            let adapter = CountingAdapter {
                native_call_count: Cell::new(0),
            };
            let output = WorthUiHostOutputEnvelope::ordinary(
                generation,
                19,
                WorthUiOrdinaryHostOutput::new(WorthUiOrdinaryHostOutputTarget::RootShell, 1),
            );
            let denial = admit_and_consume_active_host_output(&adapter, expected, &output)
                .expect_err("foreign output must be denied");
            assert_eq!(denial.reason(), expected_reason);
            assert_eq!(adapter.native_call_count.get(), 0);
        }
    }
}
