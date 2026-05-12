use forge_foundational::{
    validate_aspect_value, AspectContract, AspectValue, CanonicalF64, ContractValidatedAspectValue,
    ContractValidationDenial, ScalarAspectType,
};
use forge_proof::TransitionOutcome;

use super::support::{identity, key, revision};

#[test]
fn scalar_contract_validation_returns_proof_bearing_artifact() {
    let contract = AspectContract::scalar(
        key("temperature.celsius"),
        identity(1),
        revision(3),
        ScalarAspectType::Float64,
    );

    let outcome = validate_aspect_value(
        &contract,
        AspectValue::Float64(CanonicalF64::from_f64(21.0)).into(),
    );

    let TransitionOutcome::Success(artifact) = outcome else {
        panic!("expected validated scalar artifact");
    };

    match artifact.payload() {
        ContractValidatedAspectValue::Scalar {
            key,
            value,
            contract_revision,
        } => {
            assert_eq!(key.as_str(), "temperature.celsius");
            assert_eq!(value.value_family(), ScalarAspectType::Float64);
            assert_eq!(*contract_revision, revision(3));
        }
        ContractValidatedAspectValue::Struct { .. } => panic!("scalar contract produced struct"),
    }
}

#[test]
fn scalar_contract_validation_denies_wrong_width() {
    let contract = AspectContract::scalar(
        key("count"),
        identity(1),
        revision(1),
        ScalarAspectType::Int64,
    );

    let outcome = validate_aspect_value(&contract, AspectValue::Int32(9).into());

    assert_eq!(
        outcome,
        TransitionOutcome::Denied(ContractValidationDenial::ScalarTypeMismatch {
            expected: ScalarAspectType::Int64,
            found: ScalarAspectType::Int32,
        })
    );
}
