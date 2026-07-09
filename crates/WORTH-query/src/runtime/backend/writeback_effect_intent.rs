use std::collections::BTreeMap;

use crate::memory_workspace::WorthQueryWorkspaceError;
use crate::runtime::{WorthQueryBackendAdmissibleMutation, WorthQueryMutationFamily};
use worth_foundational::facade::{
    validate_aspect_value, AbsenceLaw, AspectContract, AspectContractRevision,
    AspectEvolutionPolicy, AspectIdentity, AspectKey, AspectMask, AspectValue,
    AuthoritativeRecordAspectPatch, FieldDeclaration, FieldKey, FieldRequirement, MutationMask,
    StructAspectShape,
};
use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::{
    BridgeWritebackEffectClass, BridgeWritebackEffectIntent, BridgeWritebackEffectIntentError,
};

pub(crate) struct WorthQueryBridgeWritebackEffectIntent {
    intent: BridgeWritebackEffectIntent,
}

impl WorthQueryBridgeWritebackEffectIntent {
    pub(crate) fn from_admitted_mutation(
        effect_class: BridgeWritebackEffectClass,
        mutation: &WorthQueryBackendAdmissibleMutation,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        let patch = authoritative_patch_from_admitted_mutation(mutation)?;
        let intent = BridgeWritebackEffectIntent::from_authoritative_patch(effect_class, patch)
            .map_err(writeback_intent_error)?;
        Ok(Self { intent })
    }

    pub(crate) fn into_bridge_intent(self) -> BridgeWritebackEffectIntent {
        self.intent
    }
}

#[derive(Default)]
struct FieldPatchDraft {
    sets: Vec<(FieldKey, AspectValue)>,
    clears: Vec<FieldKey>,
}

fn authoritative_patch_from_admitted_mutation(
    mutation: &WorthQueryBackendAdmissibleMutation,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryWorkspaceError> {
    let mut whole_sets = Vec::new();
    let mut whole_clears = Vec::new();
    let mut field_drafts = BTreeMap::<AspectKey, FieldPatchDraft>::new();

    for aspect in mutation.admitted_aspect_values() {
        let touch = aspect.aspect_touch();
        match touch.native_field_path() {
            None => {
                if aspect.clears_existing_value() {
                    whole_clears.push(touch.native_aspect_key().clone());
                } else {
                    let value = aspect.foundational_value().cloned().ok_or_else(|| {
                        WorthQueryWorkspaceError::new(
                            "set aspect mutation missing foundational value after admission",
                        )
                    })?;
                    whole_sets.push(validated_whole_aspect(
                        touch.native_aspect_key().clone(),
                        value,
                    )?);
                }
            }
            Some(field_path) => {
                let field_key = single_field_key(field_path)?;
                let draft = field_drafts
                    .entry(touch.native_aspect_key().clone())
                    .or_default();
                if aspect.clears_existing_value() {
                    draft.clears.push(field_key);
                } else {
                    let value = aspect.foundational_value().cloned().ok_or_else(|| {
                        WorthQueryWorkspaceError::new(
                            "set field mutation missing foundational value after admission",
                        )
                    })?;
                    draft.sets.push((field_key, value));
                }
            }
        }
    }

    if matches!(mutation.mutation_family(), WorthQueryMutationFamily::Delete) {
        for touch in mutation.admitted_touched_aspects() {
            match touch.native_field_path() {
                None => whole_clears.push(touch.native_aspect_key().clone()),
                Some(field_path) => {
                    let field_key = single_field_key(field_path)?;
                    field_drafts
                        .entry(touch.native_aspect_key().clone())
                        .or_default()
                        .clears
                        .push(field_key);
                }
            }
        }
    }

    let mut patch = match AuthoritativeRecordAspectPatch::whole_aspect(whole_sets, whole_clears) {
        TransitionOutcome::Success(patch) => patch,
        TransitionOutcome::Denied(denial) => {
            return Err(WorthQueryWorkspaceError::new(format!("{denial:?}")));
        }
        TransitionOutcome::Deferred(never)
        | TransitionOutcome::Stale(never)
        | TransitionOutcome::RebindRequired(never)
        | TransitionOutcome::Failed(never) => match never {},
    };

    for (aspect_key, draft) in field_drafts {
        let field_patch = field_patch(aspect_key, draft)?;
        patch = match AuthoritativeRecordAspectPatch::combine(patch, field_patch) {
            TransitionOutcome::Success(patch) => patch,
            TransitionOutcome::Denied(denial) => {
                return Err(WorthQueryWorkspaceError::new(format!("{denial:?}")));
            }
            TransitionOutcome::Deferred(never)
            | TransitionOutcome::Stale(never)
            | TransitionOutcome::RebindRequired(never)
            | TransitionOutcome::Failed(never) => match never {},
        };
    }

    Ok(patch)
}

fn validated_whole_aspect(
    aspect_key: AspectKey,
    value: AspectValue,
) -> Result<worth_foundational::facade::ContractValidatedAspectArtifact, WorthQueryWorkspaceError> {
    let contract = AspectContract::scalar(
        aspect_key,
        AspectIdentity(1),
        AspectContractRevision(1),
        value.value_family(),
    );
    match validate_aspect_value(&contract, value.into()) {
        TransitionOutcome::Success(validated) => Ok(validated),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryWorkspaceError::new(format!("{denial:?}")))
        }
        TransitionOutcome::Deferred(never)
        | TransitionOutcome::Stale(never)
        | TransitionOutcome::RebindRequired(never)
        | TransitionOutcome::Failed(never) => match never {},
    }
}

fn field_patch(
    aspect_key: AspectKey,
    draft: FieldPatchDraft,
) -> Result<AuthoritativeRecordAspectPatch, WorthQueryWorkspaceError> {
    let contract = synthetic_struct_contract(&aspect_key, &draft)?;
    let mask = AspectMask::<MutationMask>::new(
        draft
            .sets
            .iter()
            .map(|(field, _)| field.clone())
            .chain(draft.clears.iter().cloned())
            .map(|field| {
                worth_foundational::facade::CanonicalFieldPath::new([field])
                    .expect("single field key produces a canonical field path")
            }),
    );
    match AuthoritativeRecordAspectPatch::field_level(&contract, &mask, draft.sets, draft.clears) {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryWorkspaceError::new(format!("{denial:?}")))
        }
        TransitionOutcome::Deferred(never)
        | TransitionOutcome::Stale(never)
        | TransitionOutcome::RebindRequired(never)
        | TransitionOutcome::Failed(never) => match never {},
    }
}

fn synthetic_struct_contract(
    aspect_key: &AspectKey,
    draft: &FieldPatchDraft,
) -> Result<AspectContract, WorthQueryWorkspaceError> {
    let mut declarations = Vec::with_capacity(draft.sets.len() + draft.clears.len());
    for (field, value) in &draft.sets {
        declarations.push(field_declaration(
            field.clone(),
            value.value_family(),
            FieldRequirement::Optional,
        )?);
    }
    for field in &draft.clears {
        declarations.push(field_declaration(
            field.clone(),
            worth_foundational::facade::ScalarAspectType::String,
            FieldRequirement::Optional,
        )?);
    }
    let shape = StructAspectShape::new(declarations)
        .ok_or_else(|| WorthQueryWorkspaceError::new("duplicate field patch declaration"))?;
    Ok(AspectContract::struct_aspect(
        aspect_key.clone(),
        AspectIdentity(1),
        AspectContractRevision(1),
        shape,
    ))
}

fn field_declaration(
    field: FieldKey,
    value_type: worth_foundational::facade::ScalarAspectType,
    requirement: FieldRequirement,
) -> Result<FieldDeclaration, WorthQueryWorkspaceError> {
    FieldDeclaration::new(
        field,
        value_type,
        requirement,
        AbsenceLaw::Optional,
        AspectEvolutionPolicy::AdditiveFieldsAllowed,
    )
    .ok_or_else(|| WorthQueryWorkspaceError::new("invalid synthetic field declaration"))
}

fn single_field_key(
    field_path: &worth_foundational::facade::CanonicalFieldPath,
) -> Result<FieldKey, WorthQueryWorkspaceError> {
    let fields = field_path.fields();
    if fields.len() != 1 {
        return Err(WorthQueryWorkspaceError::new(
            "bridge writeback effect intent does not yet support nested field-level patches",
        ));
    }
    Ok(fields[0].clone())
}

fn writeback_intent_error(error: BridgeWritebackEffectIntentError) -> WorthQueryWorkspaceError {
    WorthQueryWorkspaceError::new(format!("{error:?}"))
}
