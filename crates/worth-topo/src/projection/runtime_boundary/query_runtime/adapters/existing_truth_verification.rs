use forge_query::facade::{
    ForgeQueryAspectValue, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeDenialKind, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeExistingTruthVerificationAdapter,
};
use serde_json::Value;

use super::binding::TopologyRuntimeBinding;
use super::query_rows::{topology_entity_rows, topology_relation_rows};

pub(crate) struct TopologyExistingTruthVerificationAdapter {
    binding: TopologyRuntimeBinding,
}

impl TopologyExistingTruthVerificationAdapter {
    pub(crate) fn new(binding: TopologyRuntimeBinding) -> Self {
        Self { binding }
    }

    fn target_row(&self, binding: &ForgeQueryExistingTruthTargetBinding) -> Option<Value> {
        match binding.target_collection() {
            Some("TopologyEntity") => topology_entity_rows(&self.binding)
                .into_iter()
                .find(|row| row.identity == binding.resolved_target_identity())
                .map(|row| row.payload),
            Some("TopologyRelation") => topology_relation_rows(&self.binding)
                .into_iter()
                .find(|row| row.identity == binding.resolved_target_identity())
                .map(|row| row.payload),
            _ => None,
        }
    }
}

impl ForgeQueryRuntimeExistingTruthVerificationAdapter
    for TopologyExistingTruthVerificationAdapter
{
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<(), ForgeQueryExistingTruthAssertionDenial> {
        let Some(payload) = self.target_row(binding) else {
            return Err(ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                "topology authoritative truth did not resolve the bound target",
            ));
        };
        for aspect in aspects {
            if aspect.clears_existing_value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported,
                    Some(aspect.aspect_path().to_string()),
                    None,
                    None,
                    "topology bridge-backed verification does not admit clear assertions",
                ));
            }
            let Some(found) = nested_value(&payload, aspect.aspect_path()) else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    None,
                    "topology authoritative truth did not contain the asserted aspect",
                ));
            };
            if found != aspect.value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    Some(found.to_string()),
                    "topology authoritative truth did not match the asserted value",
                ));
            }
        }
        Ok(())
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial> {
        let Some(payload) = self.target_row(request.binding()) else {
            return Err(ForgeQueryExistingTruthProbeDenial::new(
                request.binding(),
                ForgeQueryExistingTruthProbeDenialKind::ResolvedTargetUnavailable,
                None,
                "topology authoritative truth did not resolve the bound target",
            ));
        };
        let mut fields = Vec::with_capacity(request.aspect_paths().len());
        for aspect_path in request.aspect_paths() {
            let Some(value) = nested_value(&payload, aspect_path).cloned() else {
                return Err(ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(aspect_path.to_string()),
                    "topology authoritative truth did not contain the probed aspect",
                ));
            };
            fields.push((aspect_path.to_string(), value));
        }
        Ok(fields)
    }
}

fn nested_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}




