use worth_proof::{
    AdmitRecipeTransition, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    ContextualTransition, LowerRecipeTransition, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition, Unresolved,
};

use super::denial::SimulationHarnessBoundaryDenial;
use super::request::SimulationHarnessEntryRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SimulationHarnessEntryBasis {
    recovered_root: String,
    source_decision_digest: String,
}

impl SimulationHarnessEntryBasis {
    pub(crate) fn from_request(request: &SimulationHarnessEntryRequest) -> Self {
        Self {
            recovered_root: request.recovered_root().to_string(),
            source_decision_digest: request.source_decision_digest().to_string(),
        }
    }
}

pub(crate) fn admit_entry_request(
    request: SimulationHarnessEntryRequest,
) -> Result<SimulationHarnessEntryRequest, SimulationHarnessBoundaryDenial> {
    require_request_already_checked(&request)?;
    let basis = SimulationHarnessEntryBasis::from_request(&request);
    let unresolved = Recipe::<Unresolved, _>::new(request);
    let resolved = ResolveRecipeTransition
        .transition(
            unresolved,
            RecipeResolutionContext::new(
                basis,
                AuthorityWitness::from_authority_marker(SimulationHarnessEntryResolutionAuthority),
            ),
        )
        .into_value();
    let lowered = LowerRecipeTransition::new(CapabilityWitness::from_capability_marker(
        SimulationHarnessEntryLoweringCapability,
    ))
    .transition(resolved)
    .into_value();
    let admitted = AdmitRecipeTransition::new(AuthorityWitness::from_authority_marker(
        SimulationHarnessEntryAdmissionAuthority,
    ))
    .transition(lowered)
    .into_value();
    Ok(admitted.into_parts().0)
}

fn require_request_already_checked(
    request: &SimulationHarnessEntryRequest,
) -> Result<(), SimulationHarnessBoundaryDenial> {
    if !request.roadmap_requirements().is_complete() {
        return Err(SimulationHarnessBoundaryDenial::ProofProgressionSkipped);
    }
    Ok(())
}

struct SimulationHarnessEntryResolutionAuthority;
impl AuthorityMarker for SimulationHarnessEntryResolutionAuthority {}

struct SimulationHarnessEntryLoweringCapability;
impl CapabilityMarker for SimulationHarnessEntryLoweringCapability {}

struct SimulationHarnessEntryAdmissionAuthority;
impl AuthorityMarker for SimulationHarnessEntryAdmissionAuthority {}
