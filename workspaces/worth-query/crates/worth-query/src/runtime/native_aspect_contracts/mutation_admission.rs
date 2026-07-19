use std::collections::BTreeMap;

use worth_foundational::facade::{
    validate_aspect_value, AspectMask, AspectValue, AuthoritativeRecordAspectPatch,
    ContractValidationInput, EntityId, FieldKey, FieldLevelPatchBuilder, MutationMask,
    StructAspectValue, WholeAspectPatchBuilder,
};
use worth_proof::TransitionOutcome;

use crate::runtime::{
    WorthQueryAspectTouch, WorthQueryAuthoredAspectMutation, WorthQueryMutationFamily,
    WorthQueryWriteCommand,
};

use super::{
    WorthQueryMutationContractDenial, WorthQueryMutationContractDenialKind as DenialKind,
    WorthQueryNativeAspectContractRegistry,
};

pub(crate) fn admit_authoritative_mutation_patch(
    command: &WorthQueryWriteCommand,
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    let mut patch = match command.mutation_family() {
        WorthQueryMutationFamily::Insert => {
            admit_authored_creation_patch(command.admitted_aspect_values(), registry)?
        }
        WorthQueryMutationFamily::Assertion => {
            for authored in command.admitted_aspect_values() {
                let _ = admit_authored_value(authored, registry)?;
            }
            AuthoritativeRecordAspectPatch::empty()
        }
        WorthQueryMutationFamily::Update | WorthQueryMutationFamily::Delete => {
            admit_authored_mutation_patch(command.admitted_aspect_values(), registry)?
        }
    };
    for asserted in command.asserted_admitted_aspect_values() {
        let _ = admit_authored_value(asserted, registry)?;
    }
    for touch in command.admitted_touched_aspects() {
        let admitted = admit_clear(touch, registry)?;
        patch = combine(patch, admitted, touch.clone())?;
    }
    for symbolic in command.symbolic_aspect_references() {
        admit_symbolic_entity_reference(symbolic.aspect_touch(), registry)?;
    }

    Ok(patch)
}

pub(crate) fn admit_authored_mutation_patch(
    aspects: &[WorthQueryAuthoredAspectMutation],
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    let mut patch = AuthoritativeRecordAspectPatch::empty();
    for authored in aspects {
        let admitted = admit_authored_value(authored, registry)?;
        patch = combine(patch, admitted, authored.aspect_touch())?;
    }
    Ok(patch)
}

pub(crate) fn admit_authored_creation_patch(
    aspects: &[WorthQueryAuthoredAspectMutation],
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    let mut patch = AuthoritativeRecordAspectPatch::empty();
    let mut struct_fields = BTreeMap::<_, Vec<_>>::new();

    for authored in aspects {
        let touch = authored.aspect_touch();
        if touch.native_field_path().is_none() {
            let admitted = admit_creation_whole_value(authored, registry)?;
            patch = combine(patch, admitted, touch)?;
            continue;
        }
        let field = single_field(&touch)?;
        let value = creation_field_value(authored)?;
        struct_fields
            .entry(touch.native_aspect_key().clone())
            .or_default()
            .push((touch, field, value));
    }

    for fields in struct_fields.into_values() {
        let touch = fields[0].0.clone();
        let contract = contract_for(&touch, registry)?;
        let value =
            StructAspectValue::new(fields.into_iter().map(|(_, field, value)| (field, value)))
                .map_err(|denial_value| {
                    denial(
                        DenialKind::ContractValidationDenied,
                        touch.clone(),
                        format!("{denial_value:?}"),
                    )
                })?;
        let validated = transition(
            validate_aspect_value(contract, ContractValidationInput::Struct(value)),
            DenialKind::ContractValidationDenied,
            touch.clone(),
        )?;
        let admitted = transition(
            WholeAspectPatchBuilder::default().set(validated).finish(),
            DenialKind::AuthoritativePatchDenied,
            touch.clone(),
        )?;
        patch = combine(patch, admitted, touch)?;
    }

    Ok(patch)
}

fn admit_creation_whole_value(
    authored: &WorthQueryAuthoredAspectMutation,
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    if authored.clears_existing_value() {
        let touch = authored.aspect_touch();
        return Err(denial(
            DenialKind::ClearDuringCreation,
            touch,
            "record creation cannot clear an aspect that does not exist",
        ));
    }
    admit_authored_value(authored, registry)
}

fn creation_field_value(
    authored: &WorthQueryAuthoredAspectMutation,
) -> Result<AspectValue, WorthQueryMutationContractDenial> {
    let touch = authored.aspect_touch();
    if authored.clears_existing_value() {
        return Err(denial(
            DenialKind::ClearDuringCreation,
            touch,
            "record creation cannot clear a struct field that does not exist",
        ));
    }
    match authored.validation_input().cloned() {
        Some(ContractValidationInput::Scalar(value)) => Ok(value),
        Some(ContractValidationInput::Struct(_)) | None => Err(denial(
            DenialKind::FieldMutationRequiresScalar,
            touch,
            "struct field creation requires a native scalar field value",
        )),
    }
}

fn admit_symbolic_entity_reference(
    touch: &WorthQueryAspectTouch,
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<(), WorthQueryMutationContractDenial> {
    let contract = contract_for(touch, registry)?;
    let placeholder = AspectValue::EntityRef(EntityId::new(
        worth_foundational::facade::PartitionId::main(),
        0,
        0,
    ));
    match touch.native_field_path() {
        None => transition(
            validate_aspect_value(contract, ContractValidationInput::Scalar(placeholder))
                .map_success(|_| ()),
            DenialKind::IncompatibleSymbolicReference,
            touch.clone(),
        ),
        Some(path) => {
            let field = single_field(touch)?;
            let mask = AspectMask::<MutationMask>::new([path.clone()]);
            transition(
                FieldLevelPatchBuilder::new(contract, &mask)
                    .set_field(field, placeholder)
                    .finish()
                    .map_success(|_| ()),
                DenialKind::IncompatibleSymbolicReference,
                touch.clone(),
            )
        }
    }
}

fn admit_authored_value(
    authored: &WorthQueryAuthoredAspectMutation,
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    let touch = authored.aspect_touch();
    if authored.clears_existing_value() {
        return admit_clear(&touch, registry);
    }
    let input = authored.validation_input().cloned().ok_or_else(|| {
        denial(
            DenialKind::ContractValidationDenied,
            touch.clone(),
            "set mutation has no Foundational validation input",
        )
    })?;
    let contract = contract_for(&touch, registry)?;

    match touch.native_field_path() {
        None => {
            let validated = transition(
                validate_aspect_value(contract, input),
                DenialKind::ContractValidationDenied,
                touch.clone(),
            )?;
            transition(
                WholeAspectPatchBuilder::default().set(validated).finish(),
                DenialKind::AuthoritativePatchDenied,
                touch,
            )
        }
        Some(path) => {
            let field = single_field(&touch)?;
            let ContractValidationInput::Scalar(value) = input else {
                return Err(denial(
                    DenialKind::FieldMutationRequiresScalar,
                    touch,
                    "field-level mutation requires a native scalar field value",
                ));
            };
            let mask = AspectMask::<MutationMask>::new([path.clone()]);
            transition(
                FieldLevelPatchBuilder::new(contract, &mask)
                    .set_field(field, value)
                    .finish(),
                DenialKind::AuthoritativePatchDenied,
                touch,
            )
        }
    }
}

fn admit_clear(
    touch: &WorthQueryAspectTouch,
    registry: &WorthQueryNativeAspectContractRegistry,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    let contract = contract_for(touch, registry)?;
    match touch.native_field_path() {
        None => transition(
            WholeAspectPatchBuilder::default()
                .clear(contract.clone())
                .finish(),
            DenialKind::AuthoritativePatchDenied,
            touch.clone(),
        ),
        Some(path) => {
            let field = single_field(touch)?;
            let mask = AspectMask::<MutationMask>::new([path.clone()]);
            transition(
                FieldLevelPatchBuilder::new(contract, &mask)
                    .clear_field(field)
                    .finish(),
                DenialKind::AuthoritativePatchDenied,
                touch.clone(),
            )
        }
    }
}

fn contract_for<'a>(
    touch: &WorthQueryAspectTouch,
    registry: &'a WorthQueryNativeAspectContractRegistry,
) -> Result<&'a worth_foundational::facade::AspectContract, WorthQueryMutationContractDenial> {
    registry.contract(touch.native_aspect_key()).ok_or_else(|| {
        denial(
            DenialKind::MissingContract,
            touch.clone(),
            format!(
                "runtime has no Foundational contract for aspect `{}`",
                touch.native_aspect_key().as_str()
            ),
        )
    })
}

fn single_field(
    touch: &WorthQueryAspectTouch,
) -> Result<FieldKey, WorthQueryMutationContractDenial> {
    let fields = touch
        .native_field_path()
        .expect("field admission requires a field path")
        .fields();
    if fields.len() != 1 {
        return Err(denial(
            DenialKind::NestedFieldMutationUnsupported,
            touch.clone(),
            "Foundational field patches currently admit exactly one field below an aspect",
        ));
    }
    Ok(fields[0].clone())
}

fn combine(
    left: AuthoritativeRecordAspectPatch,
    right: AuthoritativeRecordAspectPatch,
    touch: WorthQueryAspectTouch,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryMutationContractDenial> {
    transition(
        AuthoritativeRecordAspectPatch::combine(left, right),
        DenialKind::AuthoritativePatchDenied,
        touch,
    )
}

fn transition<T, D: std::fmt::Debug>(
    outcome: TransitionOutcome<T, D>,
    kind: DenialKind,
    touch: WorthQueryAspectTouch,
) -> Result<T, WorthQueryMutationContractDenial> {
    match outcome {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial_value) => {
            Err(denial(kind, touch, format!("{denial_value:?}")))
        }
        TransitionOutcome::Deferred(never)
        | TransitionOutcome::Stale(never)
        | TransitionOutcome::RebindRequired(never)
        | TransitionOutcome::Failed(never) => match never {},
    }
}

fn denial(
    kind: DenialKind,
    touch: WorthQueryAspectTouch,
    detail: impl Into<String>,
) -> WorthQueryMutationContractDenial {
    WorthQueryMutationContractDenial::new(kind, touch, detail)
}
