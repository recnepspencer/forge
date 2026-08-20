use super::UiMountedPresentationCoordinator;

impl UiMountedPresentationCoordinator {
    pub(crate) fn admits_superseding_predecessor(
        &self,
        basis: super::super::UiMountedSupersedingPresentationBasis,
    ) -> bool {
        self.in_flight.get(&basis.attempt()).is_some_and(|state| {
            state.frame.canonical_core().frame() == basis.frame()
                && state.retention.identity() == basis.retention()
                && state.pending.iter().any(|pending| {
                    pending.token.progress_class()
                        == worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface
                })
        })
    }
}
