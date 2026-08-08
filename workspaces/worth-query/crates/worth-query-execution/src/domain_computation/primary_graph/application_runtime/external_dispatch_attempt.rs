//! Runtime-owned admission for one physical post-commit dispatch attempt.

use std::sync::Arc;

use super::{WorthQueryExternalDispatchAttemptOrdinal, WorthQueryPrimaryGraphApplicationRuntime};
use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;
use crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxObservation;
use crate::domain_computation::runtime_time::WorthQueryRuntimeClock;

/// Why this application runtime could not admit a physical dispatch attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryExternalDispatchAdmissionDenial {
    ForeignRelationalRuntime,
    AttemptIdentityExhausted,
}

/// Move-only authority for exactly one physical external dispatch attempt.
///
/// The application runtime binds the exact owner observation, Query runtime,
/// ordinal, and installed clock in one value. No caller can construct or
/// recombine those axes independently.
pub(in crate::domain_computation) struct WorthQueryAdmittedExternalDispatchAttempt {
    committed: WorthQueryCommittedDispatchOutboxObservation,
    query_runtime: WorthQueryRuntimeAuthorityIdentity,
    ordinal: WorthQueryExternalDispatchAttemptOrdinal,
    clock: Arc<WorthQueryRuntimeClock>,
}

impl WorthQueryAdmittedExternalDispatchAttempt {
    fn seal(
        committed: WorthQueryCommittedDispatchOutboxObservation,
        query_runtime: WorthQueryRuntimeAuthorityIdentity,
        ordinal: WorthQueryExternalDispatchAttemptOrdinal,
        clock: Arc<WorthQueryRuntimeClock>,
    ) -> Self {
        Self {
            committed,
            query_runtime,
            ordinal,
            clock,
        }
    }

    pub(in crate::domain_computation) const fn query_runtime(
        &self,
    ) -> WorthQueryRuntimeAuthorityIdentity {
        self.query_runtime
    }

    pub(in crate::domain_computation) const fn ordinal(
        &self,
    ) -> WorthQueryExternalDispatchAttemptOrdinal {
        self.ordinal
    }

    pub(in crate::domain_computation) fn clock(&self) -> Arc<WorthQueryRuntimeClock> {
        Arc::clone(&self.clock)
    }

    pub(in crate::domain_computation) fn into_committed(
        self,
    ) -> WorthQueryCommittedDispatchOutboxObservation {
        self.committed
    }

    #[cfg(test)]
    pub(crate) fn fixture(
        query_runtime: WorthQueryRuntimeAuthorityIdentity,
        committed: WorthQueryCommittedDispatchOutboxObservation,
        ordinal: WorthQueryExternalDispatchAttemptOrdinal,
    ) -> Self {
        Self::seal(
            committed,
            query_runtime,
            ordinal,
            Arc::new(WorthQueryRuntimeClock::system()),
        )
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema> {
    pub(in crate::domain_computation::primary_graph) fn admit_external_dispatch_attempt(
        &self,
        committed: WorthQueryCommittedDispatchOutboxObservation,
    ) -> Result<WorthQueryAdmittedExternalDispatchAttempt, WorthQueryExternalDispatchAdmissionDenial>
    {
        let relational_runtime = self
            .relational_source
            .authoritative_source_profile()
            .runtime_instance_id();
        if committed.relational_runtime_instance_id() != relational_runtime {
            return Err(WorthQueryExternalDispatchAdmissionDenial::ForeignRelationalRuntime);
        }
        let ordinal = self
            .next_external_dispatch_attempt()
            .ok_or(WorthQueryExternalDispatchAdmissionDenial::AttemptIdentityExhausted)?;
        Ok(WorthQueryAdmittedExternalDispatchAttempt::seal(
            committed,
            self.runtime.authority_identity(),
            ordinal,
            Arc::clone(&self.authorization_clock),
        ))
    }
}
