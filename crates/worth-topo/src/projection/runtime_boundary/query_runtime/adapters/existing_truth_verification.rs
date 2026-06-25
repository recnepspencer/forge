use forge_query::facade::{
    ForgeQueryAdmittedAspectValue, ForgeQueryEntity, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeDenialKind, ForgeQueryExistingTruthProbeField,
    ForgeQueryExistingTruthProbeRequest, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryRuntimeExistingTruthVerificationAdapter,
};

use super::binding::TopologyRuntimeBinding;
use super::query_rows::{topology_entity_rows, topology_relation_rows};
use crate::query_native_runtime_boundary::native_row_value_for_touch;

pub(crate) struct TopologyExistingTruthVerificationAdapter {
    binding: TopologyRuntimeBinding,
}

impl TopologyExistingTruthVerificationAdapter {
    pub(crate) fn new(binding: TopologyRuntimeBinding) -> Self {
        Self { binding }
    }

    fn target_row(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Option<ForgeQueryEntity> {
        match binding
            .target_collection_identity()
            .map(|collection| collection.as_str())
        {
            Some("TopologyEntity") => topology_entity_rows(&self.binding)
                .into_iter()
                .find(|row| row.identity() == binding.resolved_target_identity()),
            Some("TopologyRelation") => topology_relation_rows(&self.binding)
                .into_iter()
                .find(|row| row.identity() == binding.resolved_target_identity()),
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
        aspects: &[ForgeQueryAdmittedAspectValue],
    ) -> Result<(), ForgeQueryExistingTruthAssertionDenial> {
        let Some(row) = self.target_row(binding) else {
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
            let aspect_touch = aspect.aspect_touch();
            if aspect.clears_existing_value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported,
                    Some(aspect_touch),
                    None,
                    None,
                    "topology bridge-backed verification does not admit clear assertions",
                ));
            }
            let Some(expected) = aspect.foundational_value() else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect_touch),
                    None,
                    None,
                    "topology authoritative truth verification requires a native set value",
                ));
            };
            let Some(found) = native_row_value_for_touch(&row, &aspect_touch) else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect_touch),
                    Some(aspect_value_digest_for_boundary(expected)),
                    None,
                    "topology authoritative truth did not contain the asserted aspect",
                ));
            };
            if found != expected {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect_touch),
                    Some(aspect_value_digest_for_boundary(expected)),
                    Some(aspect_value_digest_for_boundary(found)),
                    "topology authoritative truth did not match the asserted value",
                ));
            }
        }
        Ok(())
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<ForgeQueryExistingTruthProbeField>, ForgeQueryExistingTruthProbeDenial> {
        let Some(row) = self.target_row(request.binding()) else {
            return Err(ForgeQueryExistingTruthProbeDenial::new(
                request.binding(),
                ForgeQueryExistingTruthProbeDenialKind::ResolvedTargetUnavailable,
                None,
                "topology authoritative truth did not resolve the bound target",
            ));
        };
        let mut fields = Vec::with_capacity(request.aspect_touches().len());
        for aspect_touch in request.aspect_touches() {
            let Some(value) = native_row_value_for_touch(&row, aspect_touch).cloned() else {
                return Err(ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(aspect_touch.clone()),
                    "topology authoritative truth did not contain the probed aspect",
                ));
            };
            fields.push(
                ForgeQueryExistingTruthProbeField::from_admitted_aspect_touch(
                    aspect_touch.clone(),
                    value,
                ),
            );
        }
        Ok(fields)
    }
}

fn aspect_value_digest_for_boundary(value: &forge_foundational::facade::AspectValue) -> String {
    format!("{value:?}")
}
