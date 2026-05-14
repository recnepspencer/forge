use forge_query::facade::{
    DeniedEffectEligibilityKind, EffectDeferredNeighborFamily, EffectDeferredResiduePosture,
    EffectDeferredSupportContract,
};

fn main() {
    let _ = EffectDeferredSupportContract {
        neighbor_family: EffectDeferredNeighborFamily::StoreBackedExecutionParity,
        denial_kind: DeniedEffectEligibilityKind::StoreBackedExecutionDeferred,
        residue_posture: EffectDeferredResiduePosture::ZeroOperationalResidue,
        contract_digest: String::new(),
    };
}
