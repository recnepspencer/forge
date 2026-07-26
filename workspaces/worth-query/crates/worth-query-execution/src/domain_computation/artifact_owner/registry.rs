use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactDisposition,
    WorthQueryArtifactLifecycleRecord, WorthQueryArtifactProductionGeneration,
    WorthQueryArtifactProductionGenerationPending, WorthQueryArtifactProviderReleasePosture,
    WorthQueryMoveOnlyArtifactHandle, WorthQueryRuntimeArtifactOwner,
    WorthQueryWorkflowArtifactRegistryEvidence,
};

pub struct WorthQueryWorkflowArtifactRegistry {
    run_identity: String,
    state: Mutex<WorthQueryWorkflowArtifactRegistryState>,
}

struct WorthQueryWorkflowArtifactRegistryState {
    posture: WorthQueryWorkflowArtifactRegistryPosture,
    owners: BTreeMap<String, WorthQueryWorkflowArtifactRegistryEntry>,
}

struct WorthQueryWorkflowArtifactRegistryEntry {
    owner: Weak<WorthQueryRuntimeArtifactOwner>,
    lifecycle: Arc<WorthQueryArtifactLifecycleRecord>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum WorthQueryWorkflowArtifactRegistryPosture {
    Producing(WorthQueryArtifactProductionGeneration),
    Frozen(WorthQueryArtifactProductionGeneration),
    ReadmissionPending {
        prior: WorthQueryArtifactProductionGeneration,
        next: WorthQueryArtifactProductionGeneration,
    },
    Closed(WorthQueryArtifactProductionGeneration),
}

impl WorthQueryWorkflowArtifactRegistry {
    pub(super) fn new(run_identity: String) -> Self {
        Self {
            run_identity,
            state: Mutex::new(WorthQueryWorkflowArtifactRegistryState {
                posture: WorthQueryWorkflowArtifactRegistryPosture::Producing(
                    WorthQueryArtifactProductionGeneration::initial(),
                ),
                owners: BTreeMap::new(),
            }),
        }
    }

    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }

    pub fn register(
        &self,
        handle: &WorthQueryMoveOnlyArtifactHandle,
        production_generation: WorthQueryArtifactProductionGeneration,
    ) -> Result<(), WorthQueryArtifactDenial> {
        if handle.core.owner.binding().run_identity != self.run_identity {
            return Err(WorthQueryArtifactDenial::new(
                WorthQueryArtifactDenialKind::RunMismatch,
                Some(
                    handle
                        .core
                        .owner
                        .binding()
                        .contract
                        .contract()
                        .family()
                        .as_str(),
                ),
                "artifact registry accepts owners from its exact workflow run",
            ));
        }
        let owner = &handle.core.owner;
        let mut state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        validate_registration_posture(state.posture, production_generation)?;
        let replaced = state.owners.insert(
            owner.binding().owner_identity.clone(),
            WorthQueryWorkflowArtifactRegistryEntry {
                owner: Arc::downgrade(owner),
                lifecycle: owner.lifecycle_record(),
            },
        );
        debug_assert!(replaced.is_none(), "artifact owner identity is unique");
        Ok(())
    }

    pub(super) fn admit_registration(
        &self,
        production_generation: WorthQueryArtifactProductionGeneration,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let posture = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .posture;
        validate_registration_posture(posture, production_generation)
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.state
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .posture
            .is_closed()
    }

    pub(crate) fn freeze_production(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        let mut state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        if let WorthQueryWorkflowArtifactRegistryPosture::Producing(generation) = state.posture {
            state.posture = WorthQueryWorkflowArtifactRegistryPosture::Frozen(generation);
        }
        drop(state);
        self.evidence()
    }

    pub fn evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        let state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        let posture = state.posture;
        let owners = state
            .owners
            .values()
            .map(|entry| Arc::clone(&entry.lifecycle))
            .collect::<Vec<_>>();
        drop(state);
        let mut retained_artifact_count = 0usize;
        let mut disposed_artifact_count = 0usize;
        let mut retained_bytes = 0usize;
        let mut provider_release_complete_count = 0usize;
        let mut provider_release_pending_count = 0usize;
        let mut provider_release_recovery_required_count = 0usize;
        for lifecycle in &owners {
            let snapshot = lifecycle.snapshot();
            if snapshot.is_disposed() {
                disposed_artifact_count = disposed_artifact_count.saturating_add(1);
            } else {
                retained_artifact_count = retained_artifact_count.saturating_add(1);
                retained_bytes = retained_bytes.saturating_add(snapshot.counters().retained_bytes);
            }
            match snapshot.provider_release() {
                WorthQueryArtifactProviderReleasePosture::Retained => {}
                WorthQueryArtifactProviderReleasePosture::Pending => {
                    provider_release_pending_count =
                        provider_release_pending_count.saturating_add(1);
                }
                WorthQueryArtifactProviderReleasePosture::Complete(_) => {
                    provider_release_complete_count =
                        provider_release_complete_count.saturating_add(1);
                }
                WorthQueryArtifactProviderReleasePosture::RecoveryRequired(_) => {
                    provider_release_recovery_required_count =
                        provider_release_recovery_required_count.saturating_add(1);
                }
            }
        }
        WorthQueryWorkflowArtifactRegistryEvidence::new(
            production_generation(posture),
            owners.len(),
            retained_artifact_count,
            disposed_artifact_count,
            retained_bytes,
            provider_release_complete_count,
            provider_release_pending_count,
            provider_release_recovery_required_count,
        )
    }

    pub(super) fn current_production_generation(
        &self,
    ) -> Result<WorthQueryArtifactProductionGeneration, WorthQueryArtifactDenial> {
        let posture = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .posture;
        match posture {
            WorthQueryWorkflowArtifactRegistryPosture::Producing(generation) => Ok(generation),
            posture => Err(registration_denial(posture)),
        }
    }

    pub(crate) fn frozen_production_generation(
        &self,
    ) -> Option<WorthQueryArtifactProductionGeneration> {
        match self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .posture
        {
            WorthQueryWorkflowArtifactRegistryPosture::Frozen(generation) => Some(generation),
            _ => None,
        }
    }

    pub(crate) fn prepare_next_generation(
        self: &Arc<Self>,
    ) -> Result<WorthQueryArtifactProductionGenerationPending, WorthQueryArtifactDenial> {
        let mut state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        let WorthQueryWorkflowArtifactRegistryPosture::Frozen(prior) = state.posture else {
            return Err(generation_transition_denial(
                "artifact production generation can advance only from a frozen yielded run",
            ));
        };
        let next = prior.next().ok_or_else(|| {
            generation_transition_denial("artifact production generation is exhausted")
        })?;
        state.posture =
            WorthQueryWorkflowArtifactRegistryPosture::ReadmissionPending { prior, next };
        Ok(WorthQueryArtifactProductionGenerationPending::new(
            Arc::clone(self),
            prior,
            next,
        ))
    }

    pub(super) fn abort_generation(
        &self,
        prior: WorthQueryArtifactProductionGeneration,
        next: WorthQueryArtifactProductionGeneration,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let mut state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        if state.posture
            != (WorthQueryWorkflowArtifactRegistryPosture::ReadmissionPending { prior, next })
        {
            return Err(generation_transition_denial(
                "artifact generation abort no longer owns the pending transition",
            ));
        }
        state.posture = WorthQueryWorkflowArtifactRegistryPosture::Frozen(prior);
        Ok(())
    }

    pub(super) fn commit_generation(
        &self,
        prior: WorthQueryArtifactProductionGeneration,
        next: WorthQueryArtifactProductionGeneration,
    ) {
        let mut state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        assert!(
            state.posture
                == (WorthQueryWorkflowArtifactRegistryPosture::ReadmissionPending { prior, next }),
            "artifact generation pending authority must exclusively own its commit transition",
        );
        state.posture = WorthQueryWorkflowArtifactRegistryPosture::Producing(next);
    }

    pub fn close_released(&self) {
        self.close(WorthQueryArtifactDisposition::Released);
    }

    pub fn close_cancelled(&self) {
        self.close(WorthQueryArtifactDisposition::Cancelled);
    }

    fn close(&self, disposition: WorthQueryArtifactDisposition) {
        let owners = {
            let mut state = self
                .state
                .lock()
                .expect("workflow artifact registry lock must remain available");
            let generation = production_generation(state.posture);
            state.posture = WorthQueryWorkflowArtifactRegistryPosture::Closed(generation);
            let owners = state
                .owners
                .values()
                .filter_map(|entry| entry.owner.upgrade())
                .collect::<Vec<_>>();
            owners
        };
        for owner in owners {
            owner.request_registry_close(disposition);
        }
    }
}

fn registration_denial(
    posture: WorthQueryWorkflowArtifactRegistryPosture,
) -> WorthQueryArtifactDenial {
    match posture {
        WorthQueryWorkflowArtifactRegistryPosture::Producing(_) => {
            unreachable!("producing registries admit artifact registration")
        }
        WorthQueryWorkflowArtifactRegistryPosture::Frozen(_)
        | WorthQueryWorkflowArtifactRegistryPosture::ReadmissionPending { .. } => {
            WorthQueryArtifactDenial::new(
                WorthQueryArtifactDenialKind::ProductionClosed,
                None,
                "workflow artifact production is frozen at a terminal lifecycle boundary",
            )
        }
        WorthQueryWorkflowArtifactRegistryPosture::Closed(_) => WorthQueryArtifactDenial::new(
            WorthQueryArtifactDenialKind::AlreadyDisposed,
            None,
            "workflow artifact registry is closed",
        ),
    }
}

fn validate_registration_posture(
    posture: WorthQueryWorkflowArtifactRegistryPosture,
    production_generation: WorthQueryArtifactProductionGeneration,
) -> Result<(), WorthQueryArtifactDenial> {
    match posture {
        WorthQueryWorkflowArtifactRegistryPosture::Producing(current)
            if current == production_generation =>
        {
            Ok(())
        }
        WorthQueryWorkflowArtifactRegistryPosture::Producing(_) => {
            Err(generation_transition_denial(
                "artifact producer belongs to a stale production generation",
            ))
        }
        posture => Err(registration_denial(posture)),
    }
}

fn production_generation(
    posture: WorthQueryWorkflowArtifactRegistryPosture,
) -> WorthQueryArtifactProductionGeneration {
    match posture {
        WorthQueryWorkflowArtifactRegistryPosture::Producing(generation)
        | WorthQueryWorkflowArtifactRegistryPosture::Frozen(generation) => generation,
        WorthQueryWorkflowArtifactRegistryPosture::ReadmissionPending { prior, .. } => prior,
        WorthQueryWorkflowArtifactRegistryPosture::Closed(generation) => generation,
    }
}

impl WorthQueryWorkflowArtifactRegistryPosture {
    const fn is_closed(self) -> bool {
        matches!(self, Self::Closed(_))
    }
}

fn generation_transition_denial(detail: &'static str) -> WorthQueryArtifactDenial {
    WorthQueryArtifactDenial::new(
        WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
        None,
        detail,
    )
}

impl Drop for WorthQueryWorkflowArtifactRegistry {
    fn drop(&mut self) {
        self.close(WorthQueryArtifactDisposition::Cancelled);
    }
}
