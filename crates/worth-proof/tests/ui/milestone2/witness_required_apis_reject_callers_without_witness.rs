use worth_proof::{AuthorityMarker, CapabilityMarker, CapabilityWitness, Recipe, AuthorityWitness};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

fn invalid_calls(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
) {
    let _missing_resolve = Recipe::new("payload").resolve_with_authority(7_u8);

    let resolved = Recipe::new("payload").resolve_with_authority(7_u8, resolution_authority);
    let _missing_lower = resolved.lower_with_capability();

    let lowered = resolved.lower_with_capability(lowering_capability);
    let _missing_admit = lowered.admit_with_authority();
}

fn main() {}
