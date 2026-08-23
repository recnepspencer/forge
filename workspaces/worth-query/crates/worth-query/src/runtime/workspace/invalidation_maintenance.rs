impl super::WorthQueryWorkspace {
    pub(crate) fn preview_projection_maintenance(
        &mut self,
        owner: &str,
        initial: &crate::projection_consumption::ConsumedProjectionFactSet,
        fresh: &crate::projection_consumption::ConsumedProjectionFactSet,
        affected_sources: std::collections::BTreeSet<String>,
        select_all_fields: bool,
        broad_projection_change: bool,
        changed_targets: &[crate::live::WorthQueryProjectionChangeTarget],
    ) -> crate::live::WorthQueryProjectionMaintenancePreview {
        self.runtime
            .granular_projection_states
            .entry(owner.to_owned())
            .or_insert_with(|| {
                crate::live::WorthQueryProjectionMaintenanceState::from_initial(initial)
            })
            .preview(
                fresh,
                affected_sources,
                select_all_fields,
                broad_projection_change,
                changed_targets,
            )
    }

    pub(crate) fn apply_projection_maintenance(
        &mut self,
        owner: &str,
        pending: crate::live::WorthQueryPendingProjectionMaintenanceState,
    ) {
        self.runtime
            .granular_projection_states
            .get_mut(owner)
            .expect("projection maintenance is applied only after its preview")
            .apply(pending);
    }
}
