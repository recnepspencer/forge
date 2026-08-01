use worth_foundational::facade::CanonicalDigestId;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
    WorthQueryInstalledGraphObligation, WorthQueryInstalledGraphObligationSetIdentity,
};

use crate::domain_computation::execution_resource_admission::{
    WorthQueryCapacityReservedExecutionResourcePlan, WorthQueryExecutionCapacityReleaseReceipt,
    WorthQueryReservedGraphProviderCapacity,
};
use crate::graph_read_access::WorthQueryGraphReadPlanReview;

use super::{WorthQueryGraphWorkIntent, WorthQueryRequiredGraphWork};

pub(super) enum WorthQueryGraphWorkAdmissionMechanics {
    ApplicationQuery {
        review: WorthQueryGraphReadPlanReview,
        capacity: WorthQueryReservedGraphProviderCapacity,
    },
    ApplicationOperation {
        capacity: Option<WorthQueryCapacityReservedExecutionResourcePlan>,
    },
}

/// Sealed, move-only authority for one capacity-reserved graph-work attempt.
///
/// Public consumers can inspect this value but cannot construct or clone it.
///
/// ```compile_fail
/// use worth_query_admission::facade::graph_obligation::WorthQueryAdmittedGraphWorkPlan;
///
/// let forged = WorthQueryAdmittedGraphWorkPlan {
///     identity: todo!(),
///     required: todo!(),
///     mechanics: todo!(),
///     canonical_work: todo!(),
/// };
/// ```
pub struct WorthQueryAdmittedGraphWorkPlan {
    identity: CanonicalDigestId,
    required: WorthQueryRequiredGraphWork,
    mechanics: WorthQueryGraphWorkAdmissionMechanics,
    canonical_work: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryAdmittedGraphWorkPlan {
    pub(super) fn seal(
        identity: CanonicalDigestId,
        required: WorthQueryRequiredGraphWork,
        mechanics: WorthQueryGraphWorkAdmissionMechanics,
        canonical_work: WorthQueryCanonicalWorkEvidence,
    ) -> Self {
        Self {
            identity,
            required,
            mechanics,
            canonical_work,
        }
    }

    pub const fn identity(&self) -> &CanonicalDigestId {
        &self.identity
    }

    pub const fn obligation_identity(&self) -> &WorthQueryInstalledGraphObligationSetIdentity {
        self.required.identity()
    }

    #[doc(hidden)]
    pub const fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        self.required.binding_identity()
    }

    pub const fn intent(&self) -> WorthQueryGraphWorkIntent {
        self.required.intent()
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.canonical_work
    }

    pub fn graph_read_review(&self) -> Option<&WorthQueryGraphReadPlanReview> {
        match &self.mechanics {
            WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { review, .. } => Some(review),
            WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation { .. } => None,
        }
    }

    pub fn execution_resources(
        &self,
    ) -> Option<&crate::domain_computation::execution_resource_admission::WorthQueryAdmittedExecutionResourcePlan>
    {
        match &self.mechanics {
            WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { .. } => None,
            WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation { capacity } => {
                capacity.as_ref().map(|capacity| capacity.resources())
            }
        }
    }

    pub fn reservation_count(&self) -> usize {
        match &self.mechanics {
            WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { .. } => 1,
            WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation { capacity } => capacity
                .as_ref()
                .map_or(0, |capacity| capacity.reservation_count()),
        }
    }

    #[doc(hidden)]
    pub fn required_obligations(&self) -> &[WorthQueryInstalledGraphObligation] {
        self.required.selected().rows()
    }

    #[doc(hidden)]
    pub fn take_operation_capacity(
        &mut self,
    ) -> Option<WorthQueryCapacityReservedExecutionResourcePlan> {
        match &mut self.mechanics {
            WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { .. } => None,
            WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation { capacity } => {
                capacity.take()
            }
        }
    }

    #[doc(hidden)]
    pub fn release(self) -> WorthQueryExecutionCapacityReleaseReceipt {
        let identity = self.identity.render_hex();
        match self.mechanics {
            WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { capacity, .. } => {
                capacity.release(&identity)
            }
            WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation { capacity } => capacity
                .expect("an untransferred operation graph plan owns its reservation")
                .release(),
        }
    }
}
