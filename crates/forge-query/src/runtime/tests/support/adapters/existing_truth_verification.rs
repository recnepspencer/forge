use super::*;

use std::collections::BTreeMap;

#[derive(Default)]
pub(in crate::runtime::tests) struct TestExistingTruthVerificationAdapter {
    values: BTreeMap<(String, String, String), Value>,
}

impl TestExistingTruthVerificationAdapter {
    pub(in crate::runtime::tests) fn with_value(
        mut self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_path: &str,
        value: Value,
    ) -> Self {
        self.values.insert(
            (
                binding.binding_digest(),
                binding.target_collection().unwrap_or("none").to_string(),
                aspect_path.to_string(),
            ),
            value,
        );
        self
    }

    fn lookup(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_path: &str,
    ) -> Option<&Value> {
        self.values.get(&(
            binding.binding_digest(),
            binding.target_collection().unwrap_or("none").to_string(),
            aspect_path.to_string(),
        ))
    }
}

impl ForgeQueryRuntimeExistingTruthVerificationAdapter for TestExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        for aspect in aspects {
            if aspect.clears_existing_value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported,
                    Some(aspect.aspect_path().to_string()),
                    None,
                    None,
                    "backend-verified assertions cannot clear authoritative truth",
                ));
            }
            let Some(found) = self.lookup(binding, aspect.aspect_path()) else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    None,
                    "authoritative truth did not contain the asserted aspect",
                ));
            };
            if found != aspect.value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    Some(found.to_string()),
                    "authoritative truth did not match the asserted value",
                ));
            }
        }

        ForgeQueryVerifiedExistingTruthAssertion::new(
            binding,
            aspects,
            crate::memory_workspace::admit_external_snapshot_label(
                "test-existing-truth-verification-snapshot",
            ),
        )
        .map_err(|error| {
            ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                error.to_string(),
            )
        })
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial> {
        let mut fields = Vec::with_capacity(request.aspect_paths().len());
        for aspect_path in request.aspect_paths() {
            let Some(value) = self.lookup(request.binding(), aspect_path) else {
                return Err(ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(aspect_path.to_string()),
                    "authoritative truth did not contain the probed aspect",
                ));
            };
            fields.push((aspect_path.clone(), value.clone()));
        }
        Ok(fields)
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct PermissiveExistingTruthVerificationAdapter;

impl ForgeQueryRuntimeExistingTruthVerificationAdapter
    for PermissiveExistingTruthVerificationAdapter
{
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        ForgeQueryVerifiedExistingTruthAssertion::new(
            binding,
            aspects,
            crate::memory_workspace::admit_external_snapshot_label(
                "permissive-existing-truth-verification-snapshot",
            ),
        )
        .map_err(|error| {
            ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                error.to_string(),
            )
        })
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial> {
        Ok(request
            .aspect_paths()
            .iter()
            .map(|aspect_path| (aspect_path.clone(), Value::String("permissive".to_string())))
            .collect())
    }
}
