impl super::UiPortalRuntimeState {
    pub(crate) fn rebind_published_presentations(
        &mut self,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        surfaces: &[crate::mounting::UiMountedSurfacePresentationReceipt],
    ) {
        for record in self.records.values_mut().filter(|record| {
            record.posture != super::UiPortalLifecyclePosture::Closed && record.placement.is_some()
        }) {
            let placement = record.placement.expect("filtered portal retains placement");
            let predecessor = placement.prepared().presentation();
            let surface = surfaces
                .iter()
                .find(|surface| surface.host_surface() == predecessor.host_surface())
                .expect("published portal overlay retains its exact host surface");
            let presentation = worth_ui_host_contract::UiHostObservationPresentationBasis::new(
                surface.host_surface(),
                frame,
                surface.binding(),
                surface.epoch(),
            );
            record.placement = Some(super::UiCommittedPortalPlacement::from_prepared(
                placement.prepared().with_presentation(presentation),
            ));
        }
    }
}
