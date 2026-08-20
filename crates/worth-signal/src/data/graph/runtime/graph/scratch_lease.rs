use crate::data::error::SignalError;
use crate::data::graph::runtime::scratch::{GraphScratch, ScratchLeaseKind, TraversalScratch};

use super::SignalGraph;

impl SignalGraph {
    pub(crate) fn acquire_scratch(
        &mut self,
        kind: ScratchLeaseKind,
    ) -> Result<TraversalScratch, SignalError> {
        let active = self.as_parts_mut().2.scratch_lease;
        if let Some(active) = active {
            if let Some(mut telemetry) = self.telemetry_mut() {
                telemetry.storage.scratch_reentry_error_count += 1;
            }
            return Err(SignalError::scratch_reentry(active, kind));
        }
        let (_, _, traversal, _) = self.as_parts_mut();
        traversal.scratch_lease = Some(kind);
        Ok(std::mem::take(&mut traversal.scratch))
    }

    pub(crate) fn restore_scratch(
        &mut self,
        kind: ScratchLeaseKind,
        scratch: TraversalScratch,
    ) -> Result<(), SignalError> {
        let (_, _, traversal, _) = self.as_parts_mut();
        match traversal.scratch_lease {
            Some(active) if active == kind => {
                traversal.scratch = scratch;
                traversal.scratch_lease = None;
                Ok(())
            }
            Some(active) => Err(SignalError::scratch_mismatch(active, kind)),
            None => Err(SignalError::internal(format!(
                "signal scratch restore called without active lease for {kind:?}"
            ))),
        }
    }

    pub(crate) fn with_scratch<R, E>(
        &mut self,
        kind: ScratchLeaseKind,
        f: impl FnOnce(&mut SignalGraph, &mut GraphScratch<'_>) -> Result<R, E>,
    ) -> Result<R, E>
    where
        E: From<SignalError>,
    {
        let mut scratch = self.acquire_scratch(kind)?;
        let mut graph_scratch = GraphScratch::new(&mut scratch);
        let result = f(self, &mut graph_scratch);
        self.restore_scratch(kind, scratch)?;
        result
    }
}
