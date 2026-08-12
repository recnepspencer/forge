use worth_store_authority::{
    RecoveryCleanupEffectAuthorization, RecoveryCleanupEffectBinding,
    StoreCurrentAuthorityWitness,
};

fn substitute_generic_authority(
    current: &StoreCurrentAuthorityWitness,
    binding: RecoveryCleanupEffectBinding,
) {
    let _ = RecoveryCleanupEffectAuthorization::issue(current, binding);
}

fn main() {}
