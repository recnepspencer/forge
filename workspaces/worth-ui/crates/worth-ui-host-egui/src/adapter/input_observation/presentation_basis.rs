#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiEguiPresentedInputBasis {
    protocol: worth_ui_host_contract::UiHostProtocolAgreement,
    host_session: u64,
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
}

impl UiEguiPresentedInputBasis {
    pub(super) fn completed(
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    ) -> Self {
        Self {
            protocol: view.protocol(),
            host_session: view.host_session_identity(),
            presentation: worth_ui_host_contract::UiHostObservationPresentationBasis::new(
                view.frame(),
                view.requirement().binding(),
                epoch,
            ),
        }
    }

    pub(super) const fn protocol(self) -> worth_ui_host_contract::UiHostProtocolAgreement {
        self.protocol
    }

    pub(super) const fn host_session(self) -> u64 {
        self.host_session
    }

    pub(super) const fn presentation(
        self,
    ) -> worth_ui_host_contract::UiHostObservationPresentationBasis {
        self.presentation
    }
}
