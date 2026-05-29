use forge_foundational::facade::{aspects, AuthoritativeRecordAspectState};
use forge_proof::TransitionOutcome;

use crate::identity::data::{EntityId, KindId};
use crate::schema::data::LoweredAspectPlan;
use crate::transactions::data::RelationAuthoritativeAspectStateDenial;

use super::authoritative_state_admission::admit_relation_creation_state;
use super::contract_validation::validate_endpoint_identity_aspects;
use super::creation_authoritative_patch::construct_creation_patch;

pub(crate) fn apply_relation_endpoint_update_aspects(
    _kind_id: KindId,
    lowered_plan: Option<&LoweredAspectPlan>,
    old_authoritative_state: Option<AuthoritativeRecordAspectState>,
    source: EntityId,
    target: EntityId,
) -> Result<Option<AuthoritativeRecordAspectState>, RelationAuthoritativeAspectStateDenial> {
    let Some(lowered_plan) = lowered_plan else {
        return Ok(old_authoritative_state);
    };
    let validated_endpoint_artifacts =
        validate_endpoint_identity_aspects(lowered_plan, source, target)?;
    let Some(endpoint_patch) = construct_creation_patch(validated_endpoint_artifacts.clone())?
    else {
        return Ok(old_authoritative_state);
    };
    let Some(old_state) = old_authoritative_state else {
        return admit_relation_creation_state(validated_endpoint_artifacts);
    };
    match aspects()
        .authoritative_state()
        .apply_patch(&old_state, &endpoint_patch)
    {
        TransitionOutcome::Success(artifact) => {
            let (state, _proofs, _basis) = artifact.into_parts().into_parts();
            Ok(Some(state))
        }
        TransitionOutcome::Denied(denial) => {
            Err(RelationAuthoritativeAspectStateDenial::PatchApplicationDenied { denial })
        }
    }
}
