use super::*;
use crate::runtime::mutation::aspect_value_native_digest_text;

use std::collections::BTreeMap;

use forge_foundational::facade::{AspectValue, InternedString};

#[derive(Default)]
pub(in crate::runtime::tests) struct TestExistingTruthVerificationAdapter {
    values: BTreeMap<(String, String, String), AspectValue>,
}

impl TestExistingTruthVerificationAdapter {
    pub(in crate::runtime::tests) fn with_value(
        mut self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_path: &str,
        value: AspectValue,
    ) -> Self {
        let aspect_touch = ForgeQueryAspectTouch::from_authoring_path(aspect_path)
            .expect("test aspect path should parse");
        self.values.insert(
            (
                binding.binding_digest(),
                binding
                    .terminal_target_collection_projection()
                    .unwrap_or("none")
                    .to_string(),
                aspect_touch.admitted_touch_digest_part(),
            ),
            value,
        );
        self
    }

    fn lookup(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_touch: &ForgeQueryAspectTouch,
    ) -> Option<&AspectValue> {
        self.values.get(&(
            binding.binding_digest(),
            binding
                .terminal_target_collection_projection()
                .unwrap_or("none")
                .to_string(),
            aspect_touch.admitted_touch_digest_part(),
        ))
    }
}

impl ForgeQueryRuntimeExistingTruthVerificationAdapter for TestExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<(), ForgeQueryExistingTruthAssertionDenial> {
        for aspect in aspects {
            let aspect_touch = aspect.aspect_touch();
            if aspect.clears_existing_value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported,
                    Some(aspect_touch.clone()),
                    None,
                    None,
                    "backend-verified assertions cannot clear authoritative truth",
                ));
            }
            let Some(found) = self.lookup(binding, &aspect_touch) else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect_touch.clone()),
                    Some(aspect.native_digest_material()),
                    None,
                    "authoritative truth did not contain the asserted aspect",
                ));
            };
            let Some(expected) = aspect.foundational_value() else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect_touch.clone()),
                    Some(aspect.native_digest_material()),
                    None,
                    "asserted aspect did not retain a native value",
                ));
            };
            if found != expected {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect_touch.clone()),
                    Some(aspect.native_digest_material()),
                    Some(native_digest_from_aspect_value(found)),
                    "authoritative truth did not match the asserted value",
                ));
            }
        }
        Ok(())
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<ForgeQueryExistingTruthProbeField>, ForgeQueryExistingTruthProbeDenial> {
        let mut fields = Vec::with_capacity(request.aspect_touches().len());
        for aspect_touch in request.aspect_touches() {
            let Some(value) = self.lookup(request.binding(), aspect_touch) else {
                return Err(ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(aspect_touch.clone()),
                    "authoritative truth did not contain the probed aspect",
                ));
            };
            let field =
                ForgeQueryExistingTruthProbeField::new_native(aspect_touch.clone(), value.clone());
            fields.push(field);
        }
        Ok(fields)
    }
}

fn native_digest_from_aspect_value(value: &AspectValue) -> String {
    aspect_value_native_digest_text(value)
}

#[derive(Default)]
pub(in crate::runtime::tests) struct PermissiveExistingTruthVerificationAdapter;

impl ForgeQueryRuntimeExistingTruthVerificationAdapter
    for PermissiveExistingTruthVerificationAdapter
{
    fn verify_existing_truth_assertion(
        &self,
        _binding: &ForgeQueryExistingTruthTargetBinding,
        _aspects: &[ForgeQueryAspectValue],
    ) -> Result<(), ForgeQueryExistingTruthAssertionDenial> {
        Ok(())
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<ForgeQueryExistingTruthProbeField>, ForgeQueryExistingTruthProbeDenial> {
        Ok(request
            .aspect_touches()
            .iter()
            .map(|aspect_touch| {
                ForgeQueryExistingTruthProbeField::new_native(
                    aspect_touch.clone(),
                    AspectValue::String(InternedString::Raw("permissive".to_string())),
                )
            })
            .collect())
    }
}
