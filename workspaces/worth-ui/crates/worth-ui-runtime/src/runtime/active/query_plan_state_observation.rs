use super::WorthUiSealedExecutionPlanBundle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveQueryPlanObservation {
    query_binding_slot_count: usize,
    missing_settled_fact_link_count: usize,
    foreign_installed_reference_count: usize,
}

impl WorthUiSealedExecutionPlanBundle {
    pub(crate) fn query_plan_state_observation(
        &self,
        binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> WorthUiActiveQueryPlanObservation {
        let plan = self.execution_plan();
        let slots = plan.regional_family_slot_view([
            crate::runtime::WorthUiPlanNodeInputFamily::QueryViewBinding,
        ]);
        let mut missing_settled_fact_link_count = 0;
        let mut foreign_installed_reference_count = 0;
        slots.for_each(|slot| {
            let Some(executable) = plan.region_store().executable_for_stable_slot(slot) else {
                missing_settled_fact_link_count += 1;
                return;
            };
            let Some(link) = executable.query_settled_fact_link() else {
                missing_settled_fact_link_count += 1;
                return;
            };
            foreign_installed_reference_count += usize::from(
                binding.reference_membership_observation(link.installed_reference())
                    != worth_ui_query_binding::WorthUiQueryReferenceMembershipObservation::ExactInstalledReference,
            );
        });
        WorthUiActiveQueryPlanObservation {
            query_binding_slot_count: slots.len(),
            missing_settled_fact_link_count,
            foreign_installed_reference_count,
        }
    }
}

impl WorthUiActiveQueryPlanObservation {
    pub(crate) fn query_binding_slot_count(self) -> usize {
        self.query_binding_slot_count
    }

    pub(crate) fn missing_settled_fact_link_count(self) -> usize {
        self.missing_settled_fact_link_count
    }

    pub(crate) fn foreign_installed_reference_count(self) -> usize {
        self.foreign_installed_reference_count
    }
}
