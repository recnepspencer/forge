mod inspection;
mod nonruntime;
mod read;
mod routing;
mod runtime;

use super::artifact::ForgeQueryIntentAdmissionPreDecisionPosture;
use super::facts::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
};
use super::request::ForgeQueryRawIntentAdmissionRequest;
use crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint;
use inspection::resolve_inspection_materialization_eligibility;
use nonruntime::{
    resolve_basis_observation_eligibility, resolve_deferred_neighbor_eligibility,
    resolve_projection_consumption_eligibility,
};
use read::resolve_read_execution_eligibility;
use routing::resolve_existing_truth_probe_routing_eligibility;
use runtime::{
    resolve_authoritative_runtime_floor_eligibility, resolve_batch_write_floor_eligibility,
    resolve_effect_runtime_floor_eligibility, resolve_scalar_write_floor_eligibility,
};

pub(super) fn resolve_eligibility_facts(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> (
    ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
) {
    match request.entrypoint() {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent => {
            resolve_authoritative_runtime_floor_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent => {
            resolve_effect_runtime_floor_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite => {
            resolve_scalar_write_floor_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite => {
            resolve_batch_write_floor_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily => {
            resolve_read_execution_eligibility(request, false, false)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext => {
            resolve_read_execution_eligibility(request, true, false)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead => {
            resolve_read_execution_eligibility(request, false, true)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection => {
            resolve_inspection_materialization_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting => {
            resolve_existing_truth_probe_routing_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation => {
            resolve_basis_observation_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption => {
            resolve_projection_consumption_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred => {
            resolve_deferred_neighbor_eligibility(
                "inspection-materialization-neighbor-deferred-until-covered",
            )
        }
    }
}
