use super::super::{WorthQueryPrimaryGraphSourceAdapter, WorthQueryPrimaryGraphSourceProjection};
use super::WorthQueryWorkspace;
use crate::memory_workspace::WorthQueryWorkspaceError;

/// Runtime-local observation that Query replaced its source adapter and
/// retained the successor primary installation.
///
/// This receipt is reconstructive evidence only. It cannot admit delivery,
/// maintenance, execution, or publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorthQueryPrimaryGraphSourceRebindReceipt {
    displaced_previous_runtime: bool,
    successor_source_readmitted: bool,
}

impl WorthQueryPrimaryGraphSourceRebindReceipt {
    pub const fn displaced_previous_runtime(&self) -> bool {
        self.displaced_previous_runtime
    }

    pub const fn successor_source_readmitted(&self) -> bool {
        self.successor_source_readmitted
    }
}

impl WorthQueryWorkspace {
    /// Rebind Query's source adapter and installed primary identity as one
    /// owner-contained reconstruction step.
    pub fn rebind_primary_graph_source<P>(
        &mut self,
        reinstallation: &worth_query_execution::facade::primary_graph::WorthQueryConditionalRuntimeReinstallationReceipt,
        projection: P,
    ) -> Result<WorthQueryPrimaryGraphSourceRebindReceipt, WorthQueryWorkspaceError>
    where
        P: WorthQueryPrimaryGraphSourceProjection,
    {
        let installation = reinstallation.successor_invalidation_installation();
        let previous = self
            .runtime
            .primary_runtime_invalidation_installation
            .as_ref()
            .ok_or_else(|| {
                WorthQueryWorkspaceError::new("Query has no incumbent primary runtime to rebind")
            })?;
        if !installation.is_immediate_successor_of(previous) {
            return Err(WorthQueryWorkspaceError::new(
                "the reinstallation receipt is not the immediate successor of Query's incumbent primary runtime",
            ));
        }
        let displaced_previous_runtime = self
            .runtime
            .primary_runtime_invalidation_installation
            .as_ref()
            .is_some_and(|previous| !previous.is_same_current_runtime_as(installation));
        let source = WorthQueryPrimaryGraphSourceAdapter::new(installation, projection);
        self.runtime
            .backend
            .rebind_primary_graph_source(installation, Box::new(source))?;
        self.runtime.primary_runtime_invalidation_installation = Some(installation.clone());
        let successor_source_readmitted = self
            .runtime
            .backend
            .readmits_primary_graph_source(installation);
        if !successor_source_readmitted {
            return Err(WorthQueryWorkspaceError::new(
                "the rebound Query source did not retain the successor primary installation",
            ));
        }
        Ok(WorthQueryPrimaryGraphSourceRebindReceipt {
            displaced_previous_runtime,
            successor_source_readmitted,
        })
    }
}
