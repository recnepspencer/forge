mod admitted_artifact;
mod birth_input;
pub(crate) mod family_birth_input;
mod placement_admission;
mod topology_ready_birth;

use self::birth_input::build_admitted_birth_input;
use self::topology_ready_birth::prepare_primitive_construction_topology_ready_birth;
use crate::construction::digest::digest_owned_parts;
use crate::construction::request::{PrimitiveConstructionPhaseError, PrimitiveConstructionRequest};

pub(crate) use self::admitted_artifact::PreparedPrimitiveConstructionAdmittedArtifact;

pub(crate) fn prepare_primitive_construction_admitted_artifact(
    request: &PrimitiveConstructionRequest,
) -> Result<PreparedPrimitiveConstructionAdmittedArtifact, PrimitiveConstructionPhaseError> {
    let intent_digest = digest_owned_parts(&[
        request.request_digest().to_string(),
        request.family().as_str().to_string(),
        "admitted".to_string(),
    ]);
    let birth_input = build_admitted_birth_input(request, &intent_digest)?;
    let topology_ready_birth = prepare_primitive_construction_topology_ready_birth(&birth_input)
        .map_err(PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff)?;
    Ok(
        PreparedPrimitiveConstructionAdmittedArtifact::from_topology_ready_birth(
            topology_ready_birth,
            birth_input.realization_report().clone(),
        ),
    )
}
