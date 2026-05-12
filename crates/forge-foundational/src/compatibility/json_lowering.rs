use forge_proof::TransitionOutcome;
use serde_json::Value;

use super::{
    JsonCompatibilityAspectInput, JsonCompatibilityLoweringDenial, JsonCompatibilityLoweringOutcome,
};
use crate::aspects::{
    admit_authoritative_record_aspect_state, validate_aspect_value, AspectContract, AspectShape,
    ContractValidatedAspectArtifact, ContractValidationDenial, FieldKey, ReferenceAspectType,
    StructAspectValue,
};
use crate::compatibility::reference_lowering::{lower_json_content_ref, lower_json_entity_ref};
use crate::compatibility::scalar_lowering::lower_json_scalar;
use crate::locators::{AspectFieldLocator, BoundarySourceLocator};

pub fn lower_json_aspect_value(
    contract: &AspectContract,
    source: BoundarySourceLocator,
    value: &Value,
) -> JsonCompatibilityLoweringOutcome<ContractValidatedAspectArtifact> {
    let lowered = match contract.shape() {
        AspectShape::Scalar(expected) => {
            lower_json_scalar(&source, value, *expected).map(Into::into)
        }
        AspectShape::Reference(ReferenceAspectType::Entity) => {
            lower_json_entity_ref(&source, value).map(Into::into)
        }
        AspectShape::Content => lower_json_content_ref(&source, value).map(Into::into),
        AspectShape::Struct(shape) => lower_json_struct(&source, value, shape).map(Into::into),
        AspectShape::Opaque(_) => Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "opaque value",
        }),
    };

    let Ok(input) = lowered else {
        return TransitionOutcome::denied(lowered.expect_err("checked above"));
    };

    match validate_aspect_value(contract, input) {
        TransitionOutcome::Success(artifact) => TransitionOutcome::success(artifact),
        TransitionOutcome::Denied(denial) => {
            TransitionOutcome::denied(JsonCompatibilityLoweringDenial::ContractValidationDenied {
                source: contract_validation_source(&source, &denial),
                denial,
            })
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => unreachable!("contract validation uses only denied"),
    }
}

pub fn lower_json_record_aspect_state(
    inputs: impl IntoIterator<Item = JsonCompatibilityAspectInput>,
) -> JsonCompatibilityLoweringOutcome<crate::aspects::AuthoritativeRecordAspectStateArtifact> {
    let mut lowered_entries = Vec::new();

    for input in inputs {
        match lower_json_aspect_value(input.contract(), input.source().clone(), input.value()) {
            TransitionOutcome::Success(artifact) => lowered_entries.push(artifact),
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            TransitionOutcome::Deferred(deferred) => return TransitionOutcome::deferred(deferred),
            TransitionOutcome::Stale(stale) => return TransitionOutcome::stale(stale),
            TransitionOutcome::RebindRequired(rebind) => {
                return TransitionOutcome::rebind_required(rebind);
            }
            TransitionOutcome::Failed(failure) => return TransitionOutcome::failed(failure),
        }
    }

    match admit_authoritative_record_aspect_state(lowered_entries) {
        TransitionOutcome::Success(state) => TransitionOutcome::success(state),
        TransitionOutcome::Denied(denial) => TransitionOutcome::denied(
            JsonCompatibilityLoweringDenial::StateAdmissionDenied(denial),
        ),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => unreachable!("state admission uses only denied"),
    }
}

fn lower_json_struct(
    source: &BoundarySourceLocator,
    value: &Value,
    shape: &crate::aspects::StructAspectShape,
) -> Result<StructAspectValue, JsonCompatibilityLoweringDenial> {
    let Value::Object(object) = value else {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "JSON object",
        });
    };

    let mut fields = Vec::new();
    for (field_name, field_value) in object {
        let Some(field_key) = FieldKey::new(field_name.clone()) else {
            return Err(JsonCompatibilityLoweringDenial::InvalidFieldKey {
                source: source.clone(),
                field: field_name.clone(),
            });
        };
        let field_source = field_source(source, &field_key);
        let Some(field) = shape.field(&field_key) else {
            return Err(JsonCompatibilityLoweringDenial::UnknownStructField {
                source: field_source,
                field: field_key,
            });
        };
        fields.push((
            field_key,
            lower_json_scalar(&field_source, field_value, field.value_type())?,
        ));
    }

    StructAspectValue::new(fields).map_err(|denial| {
        JsonCompatibilityLoweringDenial::StructConstructionDenied {
            source: source.clone(),
            denial,
        }
    })
}

fn contract_validation_source(
    source: &BoundarySourceLocator,
    denial: &ContractValidationDenial,
) -> BoundarySourceLocator {
    match denial {
        ContractValidationDenial::MissingRequiredField(field)
        | ContractValidationDenial::UnknownField(field)
        | ContractValidationDenial::FieldTypeMismatch { field, .. } => field_source(source, field),
        ContractValidationDenial::ScalarTypeMismatch { .. }
        | ContractValidationDenial::StructValueRequired
        | ContractValidationDenial::ScalarValueRequired => source.clone(),
    }
}

fn field_source(source: &BoundarySourceLocator, field_key: &FieldKey) -> BoundarySourceLocator {
    match source {
        BoundarySourceLocator::Aspect(aspect) => {
            BoundarySourceLocator::AspectField(AspectFieldLocator::new(
                aspect.authority(),
                aspect.aspect_key().clone(),
                crate::aspects::CanonicalFieldPath::single(field_key.clone()),
            ))
        }
        _ => source.clone(),
    }
}
