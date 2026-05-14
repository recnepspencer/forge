use forge_foundational::{
    admit_authoritative_record_aspect_state, aspect_state_digest_preparation_basis, AspectValue,
    ScalarAspectType,
};
use forge_proof::TransitionOutcome;

fn main() {
    let entry = {
        let contract = forge_foundational::AspectContract::scalar(
            forge_foundational::AspectKey::new("count").unwrap(),
            forge_foundational::AspectIdentity(1),
            forge_foundational::AspectContractRevision(1),
            ScalarAspectType::Int64,
        );
        let TransitionOutcome::Success(entry) =
            forge_foundational::validate_aspect_value(&contract, AspectValue::Int64(1).into())
        else {
            return;
        };
        entry
    };
    let TransitionOutcome::Success(state) = admit_authoritative_record_aspect_state([entry]) else {
        return;
    };

    let _basis = aspect_state_digest_preparation_basis(&state);
}
