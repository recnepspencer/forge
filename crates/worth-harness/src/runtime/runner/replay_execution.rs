use crate::replay::{ReplayRecord, ReplayRequest};
use crate::runtime::adapter::{HarnessAdapter, ReplayHarnessAdapter};
use crate::runtime::capability::AdapterSupport;
use crate::scenario::{MutationBatch, ScenarioFixture};

use super::core::HarnessRunner;
use super::error::HarnessError;

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + ReplayHarnessAdapter,
    A::TargetId: PartialEq,
{
    pub fn execute_replay(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        replay: &ReplayRequest<A::TargetId>,
    ) -> Result<ReplayRecord<A::TargetId>, HarnessError<A::Error>> {
        let capabilities = self.adapter.capabilities();
        if !matches!(capabilities.replay_support, AdapterSupport::Supported) {
            return Err(HarnessError::UnsupportedReplay);
        }
        let mut runtime = self
            .adapter
            .create_runtime()
            .map_err(HarnessError::Adapter)?;
        self.adapter
            .load_fixture(&mut runtime, fixture)
            .map_err(HarnessError::Adapter)?;
        if let Some(batch) = mutation_batch {
            self.adapter
                .apply_mutation_batch(&mut runtime, batch)
                .map_err(HarnessError::Adapter)?;
        }
        let mut record = self
            .adapter
            .capture_replay(&runtime, fixture, replay)
            .map_err(HarnessError::Adapter)?;
        if !replay.request.capture.mask.replay_artifacts {
            record.attachments.clear();
            record.summary = serde_json::json!({});
        }
        Ok(record)
    }
}
