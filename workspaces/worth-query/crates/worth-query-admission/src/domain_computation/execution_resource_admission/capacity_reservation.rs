use super::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryAdmittedWorkflowResourcePlan,
    WorthQueryExecutionCapacityReservation, WorthQueryExecutionResourceSupport,
};

pub struct WorthQueryCapacityReservedExecutionResourcePlan {
    resources: WorthQueryAdmittedExecutionResourcePlan,
    _provider_reservations: Vec<Box<dyn WorthQueryExecutionCapacityReservation>>,
}

impl WorthQueryCapacityReservedExecutionResourcePlan {
    fn reserve(resources: WorthQueryAdmittedExecutionResourcePlan) -> Option<Self> {
        let reservations = reserve_plans([&resources])?;
        Some(Self {
            resources,
            _provider_reservations: reservations,
        })
    }

    pub fn resources(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut WorthQueryAdmittedExecutionResourcePlan {
        &mut self.resources
    }
}

pub struct WorthQueryCapacityReservedWorkflowResourcePlan {
    resources: WorthQueryAdmittedWorkflowResourcePlan,
    _provider_reservations: Vec<Box<dyn WorthQueryExecutionCapacityReservation>>,
}

impl WorthQueryCapacityReservedWorkflowResourcePlan {
    fn reserve(resources: WorthQueryAdmittedWorkflowResourcePlan) -> Option<Self> {
        let reservations = reserve_plans(
            std::iter::once(resources.operation())
                .chain(resources.stages().map(|(_, stage)| stage)),
        )?;
        Some(Self {
            resources,
            _provider_reservations: reservations,
        })
    }

    pub fn resources(&self) -> &WorthQueryAdmittedWorkflowResourcePlan {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut WorthQueryAdmittedWorkflowResourcePlan {
        &mut self.resources
    }
}

pub fn reserve_execution_resource_plan(
    mut resources: WorthQueryAdmittedExecutionResourcePlan,
) -> Option<WorthQueryCapacityReservedExecutionResourcePlan> {
    resources.record_capacity_reservation_check();
    let mut reserved = WorthQueryCapacityReservedExecutionResourcePlan::reserve(resources)?;
    reserved.resources_mut().record_capacity_reservation();
    Some(reserved)
}

pub fn reserve_workflow_resource_plan(
    mut resources: WorthQueryAdmittedWorkflowResourcePlan,
) -> Option<WorthQueryCapacityReservedWorkflowResourcePlan> {
    resources.record_capacity_reservation_check();
    let mut reserved = WorthQueryCapacityReservedWorkflowResourcePlan::reserve(resources)?;
    reserved.resources_mut().record_capacity_reservation();
    Some(reserved)
}

fn reserve_plans<'a>(
    plans: impl IntoIterator<Item = &'a WorthQueryAdmittedExecutionResourcePlan>,
) -> Option<Vec<Box<dyn WorthQueryExecutionCapacityReservation>>> {
    let supports = exact_capacity_subjects(plans)?;
    let mut reservations = Vec::with_capacity(supports.len());
    for support in supports {
        reservations.push(support.capacity().try_reserve()?);
    }
    Some(reservations)
}

fn exact_capacity_subjects<'a>(
    plans: impl IntoIterator<Item = &'a WorthQueryAdmittedExecutionResourcePlan>,
) -> Option<Vec<&'a WorthQueryExecutionResourceSupport>> {
    let mut requested = Vec::new();
    for plan in plans {
        for support in plan.support_snapshot().all_supports() {
            retain_exact_support(&mut requested, support)?;
        }
    }
    Some(requested)
}

fn retain_exact_support<'a>(
    requested: &mut Vec<&'a WorthQueryExecutionResourceSupport>,
    support: &'a WorthQueryExecutionResourceSupport,
) -> Option<()> {
    if let Some(existing) = requested.iter().find(|existing| {
        existing.capacity_subject_identity() == support.capacity_subject_identity()
    }) {
        return existing.has_same_capacity_authority(support).then_some(());
    }
    requested.push(support);
    Some(())
}
