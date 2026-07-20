use crate::declaration::stable_text_digest;
use crate::runtime::allocation_planning::WorthUiAllocationPlanningProjection;
use crate::runtime::{
    WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAllocationPlanning {
    basis: WorthUiAllocationPlanningBasis,
    projection: Option<WorthUiAllocationPlanningProjection>,
    planning_identity_digest: u64,
    denial_posture: Option<WorthUiAllocationPlanningDenial>,
    counters: WorthUiAllocationPlanningCounters,
}

impl WorthUiAllocationPlanning {
    pub(crate) fn new(
        basis: WorthUiAllocationPlanningBasis,
        projection: Option<WorthUiAllocationPlanningProjection>,
        denial_posture: Option<WorthUiAllocationPlanningDenial>,
        counters: WorthUiAllocationPlanningCounters,
    ) -> Self {
        let planning_identity_digest = allocation_planning_identity_digest(
            &basis,
            projection.as_ref(),
            denial_posture.as_ref(),
        );
        Self {
            basis,
            projection,
            planning_identity_digest,
            denial_posture,
            counters,
        }
    }

    pub fn basis(&self) -> &WorthUiAllocationPlanningBasis {
        &self.basis
    }

    pub fn measurement_basis(&self) -> &crate::evidence::UiMeasurementBasis {
        self.basis.measurement_basis()
    }

    pub fn allocation_neighborhood(&self) -> &crate::evidence::UiAllocationNeighborhood {
        self.basis.allocation_neighborhood()
    }

    pub fn allocation_constraint_set(&self) -> Option<&crate::evidence::UiAllocationConstraintSet> {
        self.basis.allocation_constraint_set()
    }

    pub fn planning_identity_digest(&self) -> u64 {
        self.planning_identity_digest
    }

    pub fn denial_posture(&self) -> Option<&WorthUiAllocationPlanningDenial> {
        self.denial_posture.as_ref()
    }

    pub fn is_admitted(&self) -> bool {
        self.denial_posture.is_none() && self.projection.is_some()
    }

    pub fn counters(&self) -> WorthUiAllocationPlanningCounters {
        self.counters
    }

    pub(crate) fn projection(&self) -> Option<&WorthUiAllocationPlanningProjection> {
        self.projection.as_ref()
    }
}

fn allocation_planning_identity_digest(
    basis: &WorthUiAllocationPlanningBasis,
    projection: Option<&WorthUiAllocationPlanningProjection>,
    denial_posture: Option<&WorthUiAllocationPlanningDenial>,
) -> u64 {
    let mut digest = stable_text_digest("worth-ui.runtime.allocation-planning")
        ^ basis.measurement_basis().identity_digest().rotate_left(7)
        ^ basis
            .allocation_neighborhood()
            .identity()
            .identity_digest()
            .rotate_left(13);

    if let Some(constraint_set) = basis.allocation_constraint_set() {
        digest ^= constraint_set.identity().identity_digest().rotate_left(17);
    }
    if let Some(portal) = basis.portal_allocation_input() {
        digest ^= portal.identity_digest().rotate_left(37);
    }

    if let Some(projection) = projection {
        digest ^= projection.evidence_digest().rotate_left(19);
    }

    if let Some(denial) = denial_posture {
        digest ^= (denial.reason() as u64).rotate_left(31);
    }

    digest
}
