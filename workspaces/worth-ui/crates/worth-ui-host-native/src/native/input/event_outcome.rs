use super::{
    UiNativeInputObservationDisposition, UiNativeInputObservationState,
    UiNativeInputObservationStop,
};

impl UiNativeInputObservationState {
    pub(super) fn rejection_disposition(&self) -> UiNativeInputObservationDisposition {
        if self.terminal_stop.is_some() {
            UiNativeInputObservationDisposition::Stopped
        } else {
            UiNativeInputObservationDisposition::Ignored
        }
    }

    pub(super) fn terminal_disposition(
        &mut self,
        stop: UiNativeInputObservationStop,
    ) -> UiNativeInputObservationDisposition {
        self.record_terminal_stop(stop);
        UiNativeInputObservationDisposition::Stopped
    }

    pub(super) fn denial_disposition(
        &mut self,
        stop: UiNativeInputObservationStop,
    ) -> UiNativeInputObservationDisposition {
        self.record_stop(stop);
        UiNativeInputObservationDisposition::Ignored
    }
}

pub(super) fn input_affine_batch_fits(
    payloads: &[worth_ui_host_contract::UiHostObservationPayload],
) -> bool {
    payloads
        .iter()
        .map(worth_ui_host_contract::UiHostObservationReport::input_affine_encoded_len)
        .sum::<usize>()
        <= worth_ui_host_contract::UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT
}
