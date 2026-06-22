use forge_query::facade::{
    ForgeQueryAspectValue, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeDenialKind, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQuerySnapshotIdentity,
    ForgeQueryVerifiedExistingTruthAssertion,
};
use serde_json::Value;

use super::SharedRuntimeState;

pub(super) struct PublicExistingTruthVerificationAdapter {
    state: SharedRuntimeState,
}

impl PublicExistingTruthVerificationAdapter {
    pub(super) fn new(state: SharedRuntimeState) -> Self {
        Self { state }
    }
}

impl forge_query::facade::ForgeQueryRuntimeExistingTruthVerificationAdapter
    for PublicExistingTruthVerificationAdapter
{
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        let state = self.state.borrow();
        for aspect in aspects {
            let key = existing_truth_key(binding, aspect.aspect_path());
            let Some(found) = state.existing_truth_values.get(&key) else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    None,
                    "public bridge verification state did not contain the asserted aspect",
                ));
            };
            if found != aspect.value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    Some(found.to_string()),
                    "public bridge verification state did not match the asserted value",
                ));
            }
        }
        ForgeQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
            binding,
            aspects,
            &ForgeQuerySnapshotIdentity::admit_external_token(
                forge_query::facade::QueryExternalIdentityToken::new(std::sync::Arc::from(
                    "public-bridge-existing-truth-snapshot",
                )),
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
        let state = self.state.borrow();
        let mut fields = Vec::with_capacity(request.aspect_paths().len());
        for aspect_path in request.aspect_paths() {
            let key = existing_truth_key(request.binding(), aspect_path);
            let Some(value) = state.existing_truth_values.get(&key) else {
                return Err(ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(aspect_path.to_string()),
                    "public bridge verification state did not contain the probed aspect",
                ));
            };
            fields.push((aspect_path.clone(), value.clone()));
        }
        Ok(fields)
    }
}

fn existing_truth_key(
    binding: &ForgeQueryExistingTruthTargetBinding,
    aspect_path: &str,
) -> (String, String, String) {
    (
        binding.binding_digest(),
        binding.target_collection().unwrap_or("none").to_string(),
        aspect_path.to_string(),
    )
}
