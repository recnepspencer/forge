use std::collections::{BTreeMap, BTreeSet};

use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;

use super::super::SemanticOutputKey;
use super::runtime_finance::runtime_financial_snapshot;
use super::CompiledFinancialWorld;
use crate::tests::domains::fintech::world::baseline::verify_projected_work;

impl CompiledFinancialWorld {
    pub(in crate::tests::domains::fintech) fn baseline_dependency_revision(
        &self,
        key: SemanticOutputKey,
    ) -> u64 {
        self.baseline_dependency_revisions[&key]
    }

    pub(in crate::tests::domains::fintech) fn baseline_node_version(
        &self,
        key: SemanticOutputKey,
    ) -> &AspectVersion {
        &self.baseline_aspect_versions[&key]
    }

    pub(crate) fn verify_committed_financial_truth(
        &self,
        required_work: &BTreeSet<SemanticOutputKey>,
    ) -> Result<(), SignalError> {
        let fresh = runtime_financial_snapshot(&self.definition);
        if self.economic_snapshot != fresh {
            return Err(SignalError::internal(
                "compiled financial snapshot disagrees with authoritative definition",
            ));
        }
        verify_projected_work(self, required_work)
    }

    pub(crate) fn committed_financial_values(
        &self,
    ) -> Result<BTreeMap<SemanticOutputKey, i64>, SignalError> {
        self.projection
            .iter()
            .map(|(key, _)| {
                let identity = self
                    .runtime
                    .graph()
                    .node_runtime_artifact_warm(self.handles.node_for(key))?
                    .and_then(|warm| warm.output_identity.as_ref())
                    .ok_or_else(|| {
                        SignalError::internal(format!(
                            "financial node {key:?} lacks a committed output identity"
                        ))
                    })?;
                let (_, value) = identity.as_str().rsplit_once(':').ok_or_else(|| {
                    SignalError::internal(format!(
                        "financial node {key:?} has a malformed output identity"
                    ))
                })?;
                let value = value.parse::<i64>().map_err(|_| {
                    SignalError::internal(format!(
                        "financial node {key:?} has a non-numeric output identity"
                    ))
                })?;
                Ok((key, value))
            })
            .collect()
    }

    #[cfg(test)]
    pub(in crate::tests::domains::fintech) fn forge_node_version_for_test(
        &mut self,
        key: SemanticOutputKey,
        version: AspectVersion,
    ) -> Result<(), SignalError> {
        let node = self.handles.node_for(key);
        self.runtime
            .graph_mut()
            .get_entry_mut(node)?
            .set_aspect_version(version);
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::tests::domains::fintech) fn advance_dependency_revision_for_test(
        &mut self,
        key: SemanticOutputKey,
    ) -> Result<(), SignalError> {
        let node = self.handles.node_for(key);
        self.runtime
            .graph_mut()
            .get_entry_mut(node)?
            .advance_dependency_revision();
        Ok(())
    }
}
