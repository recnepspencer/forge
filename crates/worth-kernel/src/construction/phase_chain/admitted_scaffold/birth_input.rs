use super::family_birth_input::build_family_birth_input;
use super::placement_admission::admit_request_placement;
use crate::construction::request::{PrimitiveConstructionPhaseError, PrimitiveConstructionRequest};
use worth_spatial::facade::bindings::PrimitiveConstructionBirthScaffoldInput;

pub(super) fn build_admitted_birth_input(
    request: &PrimitiveConstructionRequest,
    intent_digest: &str,
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    let admitted_placement = admit_request_placement(request)?;
    build_family_birth_input(request, &admitted_placement, intent_digest)
}
