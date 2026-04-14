use crate::{
    evidence::{OperatingModeLane, PersistedModeLaneEvidence},
    facade::{ForgeStore, ForgeStoreBuilder},
    failure::{StoreError, StoreErrorKind},
    modes::lifecycle::{DurableModeConstructionPlan, HostedRuntimeOwnershipProof},
    publication::{default_runtime_session_id, execute_durable_publication},
    recovery::{DurableRecoveryOutcome, DurableRecoveryPlan},
    wal::DurableMutationId,
};
use forge_relational::facade::{history::CommitId, runtime::RelationalRuntime};

#[derive(Debug, Clone)]
pub struct AcknowledgedDurableCommit {
    durable_mutation_id: DurableMutationId,
    persisted: crate::PersistedAuthoritativeCommit,
}

impl AcknowledgedDurableCommit {
    fn new(
        durable_mutation_id: DurableMutationId,
        persisted: crate::PersistedAuthoritativeCommit,
    ) -> Self {
        Self {
            durable_mutation_id,
            persisted,
        }
    }

    pub fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }

    pub fn persisted(&self) -> &crate::PersistedAuthoritativeCommit {
        &self.persisted
    }
}

#[derive(Debug)]
pub struct DurableModeBuilder {
    construction: DurableModeConstructionPlan,
}

impl DurableModeBuilder {
    pub(crate) fn new(store_builder: ForgeStoreBuilder, runtime: RelationalRuntime) -> Self {
        Self {
            construction: DurableModeConstructionPlan::new(store_builder, runtime),
        }
    }

    pub fn build_pending(self) -> Result<DurableRecoveryHandle, StoreError> {
        let (store_builder, ownership) = self.construction.into_parts();
        let store = store_builder.build()?;
        store.record_durable_mode_selection();
        store.record_hosted_runtime_start();
        Ok(DurableRecoveryHandle {
            store,
            ownership,
            runtime_session_id: default_runtime_session_id().to_string(),
        })
    }

    pub fn build(self) -> Result<DurableStoreHandle, StoreError> {
        self.build_pending()?.recover()
    }
}

pub struct DurableMutationRequest<F> {
    operation_name: String,
    execute: F,
}

impl<F> DurableMutationRequest<F> {
    pub fn new(operation_name: impl Into<String>, execute: F) -> Self {
        Self {
            operation_name: operation_name.into(),
            execute,
        }
    }

    fn into_parts(self) -> (String, F) {
        (self.operation_name, self.execute)
    }
}

impl<F> std::fmt::Debug for DurableMutationRequest<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DurableMutationRequest")
            .field("operation_name", &self.operation_name)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct DurableRecoveryHandle {
    store: ForgeStore,
    ownership: HostedRuntimeOwnershipProof,
    runtime_session_id: String,
}

impl DurableRecoveryHandle {
    pub fn plan(&self) -> DurableRecoveryPlan {
        self.store.plan_durable_recovery()
    }

    pub fn recover(mut self) -> Result<DurableStoreHandle, StoreError> {
        let _plan = self.plan();
        let recovery = self
            .store
            .recover_durable_runtime(&self.runtime_session_id)?;
        Ok(DurableStoreHandle {
            store: self.store,
            ownership: self.ownership,
            runtime_session_id: self.runtime_session_id,
            last_recovery: recovery,
        })
    }
}

#[derive(Debug)]
pub struct DurableStoreHandle {
    store: ForgeStore,
    ownership: HostedRuntimeOwnershipProof,
    runtime_session_id: String,
    last_recovery: DurableRecoveryOutcome,
}

impl DurableStoreHandle {
    pub fn execute_mutation<F>(
        &mut self,
        request: DurableMutationRequest<F>,
    ) -> Result<AcknowledgedDurableCommit, StoreError>
    where
        F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
    {
        self.execute_mutation_internal(request, None)?
            .1
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::AcknowledgmentBoundaryViolation,
                    "durable mutation terminated before acknowledgment",
                )
            })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_mutation_until_crash<F>(
        &mut self,
        request: DurableMutationRequest<F>,
        crash_point: crate::publication::SimulatedCrashPoint,
    ) -> Result<DurableMutationId, StoreError>
    where
        F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
    {
        let (durable_mutation_id, _) =
            self.execute_mutation_internal(request, Some(crash_point))?;
        Ok(durable_mutation_id)
    }

    pub fn store(&self) -> &ForgeStore {
        &self.store
    }

    pub fn milestone_2_lane_evidence(&self) -> PersistedModeLaneEvidence {
        self.store
            .milestone_2_lane_evidence(OperatingModeLane::Durable)
    }

    pub fn hosted_runtime(&self) -> &RelationalRuntime {
        self.ownership.runtime()
    }

    pub fn last_recovery(&self) -> &DurableRecoveryOutcome {
        &self.last_recovery
    }

    pub fn resolve_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<crate::DurableRetryResolution, StoreError> {
        self.store.resolve_durable_retry(durable_mutation_id)
    }

    pub fn shutdown(self) -> (ForgeStore, RelationalRuntime) {
        self.store.record_hosted_runtime_stop();
        (self.store, self.ownership.into_runtime())
    }

    fn execute_mutation_internal<F>(
        &mut self,
        request: DurableMutationRequest<F>,
        crash_point: Option<crate::publication::SimulatedCrashPoint>,
    ) -> Result<(DurableMutationId, Option<AcknowledgedDurableCommit>), StoreError>
    where
        F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
    {
        let (operation_name, execute) = request.into_parts();
        let publication_result = execute_durable_publication(
            &mut self.store,
            &mut self.ownership,
            &self.runtime_session_id,
            operation_name,
            execute,
            crash_point,
        )?;
        let durable_mutation_id = publication_result.durable_mutation_id();
        let persisted = publication_result.into_persisted();
        Ok((
            durable_mutation_id,
            persisted
                .map(|persisted| AcknowledgedDurableCommit::new(durable_mutation_id, persisted)),
        ))
    }
}
