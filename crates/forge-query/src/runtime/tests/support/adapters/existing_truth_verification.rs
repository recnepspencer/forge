use super::*;
use crate::runtime::mutation::terminal_aspect_value_digest_text;

use std::collections::BTreeMap;

use forge_foundational::facade::AspectValue;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TestExistingTruthKey {
    binding_digest: String,
    target_collection: String,
    aspect_touch: ForgeQueryAspectTouch,
}

impl TestExistingTruthKey {
    fn new(
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_touch: ForgeQueryAspectTouch,
    ) -> Self {
        Self {
            binding_digest: binding.binding_digest(),
            target_collection: binding
                .terminal_target_collection_projection()
                .unwrap_or("none")
                .to_string(),
            aspect_touch,
        }
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct TestExistingTruthVerificationAdapter {
    values: BTreeMap<TestExistingTruthKey, AspectValue>,
}

impl TestExistingTruthVerificationAdapter {
    pub(in crate::runtime::tests) fn with_value(
        mut self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        touch_fixture: &str,
        value: AspectValue,
    ) -> Self {
        let aspect_touch = test_aspect_touch(touch_fixture);
        self.values
            .insert(TestExistingTruthKey::new(binding, aspect_touch), value);
        self
    }

    fn lookup(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_touch: &ForgeQueryAspectTouch,
    ) -> Option<&AspectValue> {
        self.values
            .get(&TestExistingTruthKey::new(binding, aspect_touch.clone()))
    }
}

impl ForgeQueryRuntimeExistingTruthVerificationAdapter for TestExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAdmittedAspectValue],
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
                    Some(aspect.terminal_digest_material()),
                    None,
                    "authoritative truth did not contain the asserted aspect",
                ));
            };
            let Some(expected) = aspect.foundational_value() else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect_touch.clone()),
                    Some(aspect.terminal_digest_material()),
                    None,
                    "asserted aspect did not retain a native value",
                ));
            };
            if found != expected {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect_touch.clone()),
                    Some(aspect.terminal_digest_material()),
                    Some(terminal_digest_from_aspect_value(found)),
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
            let field = ForgeQueryExistingTruthProbeField::from_admitted_aspect_touch(
                aspect_touch.clone(),
                value.clone(),
            );
            fields.push(field);
        }
        Ok(fields)
    }
}

fn terminal_digest_from_aspect_value(value: &AspectValue) -> String {
    terminal_aspect_value_digest_text(value)
}

#[derive(Default)]
pub(in crate::runtime::tests) struct PermissiveExistingTruthVerificationAdapter;

impl ForgeQueryRuntimeExistingTruthVerificationAdapter
    for PermissiveExistingTruthVerificationAdapter
{
    fn verify_existing_truth_assertion(
        &self,
        _binding: &ForgeQueryExistingTruthTargetBinding,
        _aspects: &[ForgeQueryAdmittedAspectValue],
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
                ForgeQueryExistingTruthProbeField::from_admitted_aspect_touch(
                    aspect_touch.clone(),
                    crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                        "permissive",
                    ),
                )
            })
            .collect())
    }
}
