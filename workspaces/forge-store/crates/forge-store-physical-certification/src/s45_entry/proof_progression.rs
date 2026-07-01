use forge_proof::{
    AdmitRecipeTransition, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    ContextualTransition, LowerRecipeTransition, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition, Unresolved,
};

use super::denial::S45HarnessBoundaryDenial;
use super::request::S45HarnessEntryRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S45HarnessEntryBasis {
    recovered_root: String,
    source_decision_digest: String,
    required_lanes: usize,
}

impl S45HarnessEntryBasis {
    pub(crate) fn from_request(request: &S45HarnessEntryRequest) -> Self {
        Self {
            recovered_root: request.recovered_root().to_string(),
            source_decision_digest: request.source_decision_digest().to_string(),
            required_lanes: request.s4_required_lanes(),
        }
    }
}

pub(crate) fn admit_entry_request(
    request: S45HarnessEntryRequest,
) -> Result<S45HarnessEntryRequest, S45HarnessBoundaryDenial> {
    require_request_already_checked(&request)?;
    let basis = S45HarnessEntryBasis::from_request(&request);
    let unresolved = Recipe::<Unresolved, _>::new(request);
    let resolved = ResolveRecipeTransition
        .transition(
            unresolved,
            RecipeResolutionContext::new(
                basis,
                AuthorityWitness::from_authority_marker(S45HarnessEntryResolutionAuthority),
            ),
        )
        .into_value();
    let lowered = LowerRecipeTransition::new(CapabilityWitness::from_capability_marker(
        S45HarnessEntryLoweringCapability,
    ))
    .transition(resolved)
    .into_value();
    let admitted = AdmitRecipeTransition::new(AuthorityWitness::from_authority_marker(
        S45HarnessEntryAdmissionAuthority,
    ))
    .transition(lowered)
    .into_value();
    Ok(admitted.into_parts().0)
}

fn require_request_already_checked(
    request: &S45HarnessEntryRequest,
) -> Result<(), S45HarnessBoundaryDenial> {
    if request.s4_completed_lanes() != request.s4_required_lanes()
        || !request.roadmap_requirements().is_complete()
    {
        return Err(S45HarnessBoundaryDenial::ProofProgressionSkipped);
    }
    Ok(())
}

struct S45HarnessEntryResolutionAuthority;
impl AuthorityMarker for S45HarnessEntryResolutionAuthority {}

struct S45HarnessEntryLoweringCapability;
impl CapabilityMarker for S45HarnessEntryLoweringCapability {}

struct S45HarnessEntryAdmissionAuthority;
impl AuthorityMarker for S45HarnessEntryAdmissionAuthority {}
