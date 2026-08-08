mod bulk_admission;
mod bulk_planning;
mod staging_inspection;
mod validated_mutation;
mod validated_mutation_footprint;

pub use validated_mutation::{RelationalMutationInvariantEvidence, ValidatedRelationalMutation};
pub use validated_mutation_footprint::ValidatedMutationFootprint;

use crate::capabilities::RuntimeConfigSource;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitConflict, LineageSafeBulkMutationBatch, MergedCommitPlan, NamingStableBulkMutationBatch,
    PlannedBulkMutationBatch, ProvenanceCompleteBulkMutationBatch, SavepointId, TransactionOptions,
    WorkerIntentBatch,
};

pub struct RelationalTransaction<'a> {
    pub(crate) runtime: &'a mut RelationalRuntime,
    pub(crate) transaction_id: crate::transactions::data::TransactionId,
    pub(crate) options: TransactionOptions,
    pub(crate) batches: Vec<WorkerIntentBatch>,
    pub(crate) savepoints: Vec<(SavepointId, usize)>,
    pub(crate) last_merged_plan: Option<MergedCommitPlan>,
}

impl<'a> RelationalTransaction<'a> {
    pub fn plan_bulk_mutation_batch(&self) -> Option<PlannedBulkMutationBatch> {
        let intents = bulk_planning::canonical_bulk_mutation_intents(
            &self.batches,
            self.runtime
                .runtime_config()
                .identity
                .client_key_symbol_policy,
            self.runtime.services.symbols.clone(),
        );
        if intents.is_empty() {
            return None;
        }

        Some(PlannedBulkMutationBatch {
            transaction_id: self.transaction_id,
            scope: bulk_planning::bulk_mutation_scope(&intents),
            locality: bulk_planning::bulk_mutation_locality(&intents),
            naming: bulk_planning::bulk_mutation_naming(&intents),
            lineage: bulk_planning::bulk_mutation_lineage(&intents),
            provenance: bulk_planning::bulk_mutation_provenance(
                self.transaction_id,
                self.options.target_branch.clone(),
                &self.batches,
            ),
            intents: intents.into(),
        })
    }

    pub fn admit_naming_stable_bulk_mutation_batch(
        &self,
    ) -> Result<Option<NamingStableBulkMutationBatch>, CommitConflict> {
        let Some(planned) = self.plan_bulk_mutation_batch() else {
            return Ok(None);
        };
        bulk_admission::validate_naming_plan(
            &planned,
            self.runtime
                .runtime_config()
                .identity
                .client_key_symbol_policy,
        )?;
        Ok(Some(
            crate::transactions::data::naming_stable_bulk_mutation_batch(planned),
        ))
    }

    pub fn admit_lineage_safe_bulk_mutation_batch(
        &self,
    ) -> Result<Option<LineageSafeBulkMutationBatch>, CommitConflict> {
        let Some(naming_stable) = self.admit_naming_stable_bulk_mutation_batch()? else {
            return Ok(None);
        };
        bulk_admission::validate_lineage_plan(naming_stable.planned())?;
        Ok(Some(
            crate::transactions::data::lineage_safe_bulk_mutation_batch(naming_stable),
        ))
    }

    pub fn admit_provenance_complete_bulk_mutation_batch(
        &self,
    ) -> Result<Option<ProvenanceCompleteBulkMutationBatch>, CommitConflict> {
        let Some(lineage_safe) = self.admit_lineage_safe_bulk_mutation_batch()? else {
            return Ok(None);
        };
        bulk_admission::validate_provenance_plan(lineage_safe.planned(), &self.batches)?;
        Ok(Some(
            crate::transactions::data::provenance_complete_bulk_mutation_batch(lineage_safe),
        ))
    }

    pub fn inspect_staging(&self) -> crate::inspection::data::TransactionInspectionSurface {
        staging_inspection::inspect_staging_surface(
            self.transaction_id,
            self.options.target_branch.clone(),
            &self.savepoints,
            &self.batches,
        )
    }
}
