use crate::evidence::sha256_serialized;

use super::execution_plan::PlanDigestBasis;
use super::SelectedProofExecutionPlan;

impl SelectedProofExecutionPlan {
    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.product != self.request.display_name()
            || self.maximum_concurrency == 0
            || self.maximum_concurrency > 8
            || self
                .structural_preflight
                .evidence_identity
                .trim()
                .is_empty()
            || self.structural_preflight.bundle_path.trim().is_empty()
            || self.structural_preflight.predicates.is_empty()
            || self.units.iter().any(|unit| {
                unit.package.trim().is_empty()
                    || unit.target_name.trim().is_empty()
                    || unit.resources.target_root.trim().is_empty()
                    || unit.timeout_millis == 0
                    || unit.expected_evidence.is_empty()
            })
        {
            return Err(
                "selected proof execution plan has an incomplete authority surface".to_owned(),
            );
        }
        if self.expected_digest()? != self.plan_digest {
            return Err(
                "selected proof execution plan digest does not match its contents".to_owned(),
            );
        }
        Ok(())
    }

    fn expected_digest(&self) -> Result<String, String> {
        sha256_serialized(&PlanDigestBasis {
            request: &self.request,
            repository: &self.repository,
            selection: &self.selection,
            units: &self.units,
            ci_shard_plan: &self.ci_shard_plan,
            maximum_concurrency: self.maximum_concurrency,
            failure_policy: self.failure_policy,
            structural_preflight: &self.structural_preflight,
            source_edit: &self.source_edit,
        })
    }
}
