use core::convert::Infallible;

use worth_proof::prelude::{AuthorityMarker, AuthorityWitness, ProofOutcome};

use crate::{IoSchedulerIsolationAdmission, IoSchedulerIsolationCounterSnapshot};

use super::{BackgroundIoPressureClass, BackgroundPacingDenial};

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundPacingAuthority {
    _private: (),
}

impl AuthorityMarker for BackgroundPacingAuthority {}

impl BackgroundPacingAuthority {
    pub(crate) fn store_owned() -> AuthorityWitness<Self> {
        AuthorityWitness::from_authority_marker(Self { _private: () })
    }
}

pub type BackgroundPacingProgressionOutcome = ProofOutcome<
    BackgroundPacingReady,
    BackgroundPacingDenial,
    BackgroundPacingDeferredProof,
    BackgroundPacingStale,
    BackgroundPacingRebindRequired,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundPacingFreshness {
    Current,
    Deferred,
    Denied,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundPacingProgressionDrift {
    DeferredReadinessCounters,
    DeniedReadinessCounters,
    StaleReadinessCounters,
    RebindRequiredReadinessCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingProgressionEvidence {
    freshness: BackgroundPacingFreshness,
    admitted_counters: IoSchedulerIsolationCounterSnapshot,
    observed_counters: IoSchedulerIsolationCounterSnapshot,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundPacingReady {
    class: BackgroundIoPressureClass,
    authority_witness: AuthorityWitness<BackgroundPacingAuthority>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingDeferredProof {
    class: BackgroundIoPressureClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingStale {
    class: BackgroundIoPressureClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingRebindRequired {
    class: BackgroundIoPressureClass,
}

pub(crate) fn prove_background_pacing_current(
    class: BackgroundIoPressureClass,
    freshness: BackgroundPacingFreshness,
) -> BackgroundPacingProgressionOutcome {
    let outcome = match freshness {
        BackgroundPacingFreshness::Current => worth_proof::TransitionOutcome::<
            BackgroundPacingReady,
            BackgroundPacingDenial,
            BackgroundPacingDeferredProof,
            BackgroundPacingStale,
            BackgroundPacingRebindRequired,
            Infallible,
        >::success(BackgroundPacingReady {
            class,
            authority_witness: BackgroundPacingAuthority::store_owned(),
        }),
        BackgroundPacingFreshness::Deferred => {
            worth_proof::TransitionOutcome::deferred(BackgroundPacingDeferredProof { class })
        }
        BackgroundPacingFreshness::Denied => worth_proof::TransitionOutcome::denied(
            BackgroundPacingDenial::PacingProgressionDenied(class),
        ),
        BackgroundPacingFreshness::Stale => {
            worth_proof::TransitionOutcome::stale(BackgroundPacingStale { class })
        }
        BackgroundPacingFreshness::RebindRequired => {
            worth_proof::TransitionOutcome::rebind_required(BackgroundPacingRebindRequired {
                class,
            })
        }
        BackgroundPacingFreshness::Failed => worth_proof::TransitionOutcome::denied(
            BackgroundPacingDenial::PacingProgressionFailed(class),
        ),
    };
    outcome.into()
}

impl BackgroundPacingReady {
    pub const fn authority_witness(&self) -> &AuthorityWitness<BackgroundPacingAuthority> {
        &self.authority_witness
    }
}

impl BackgroundPacingProgressionEvidence {
    pub fn current(readiness: &IoSchedulerIsolationAdmission) -> Self {
        let counters = readiness.counters();
        Self {
            freshness: BackgroundPacingFreshness::Current,
            admitted_counters: counters,
            observed_counters: counters,
        }
    }

    pub fn from_readiness_counter_drift(
        readiness: &IoSchedulerIsolationAdmission,
        observed_counters: IoSchedulerIsolationCounterSnapshot,
        drift: BackgroundPacingProgressionDrift,
    ) -> Option<Self> {
        let admitted_counters = readiness.counters();
        if admitted_counters == observed_counters {
            return None;
        }
        Some(Self {
            freshness: match drift {
                BackgroundPacingProgressionDrift::DeferredReadinessCounters => {
                    BackgroundPacingFreshness::Deferred
                }
                BackgroundPacingProgressionDrift::DeniedReadinessCounters => {
                    BackgroundPacingFreshness::Denied
                }
                BackgroundPacingProgressionDrift::StaleReadinessCounters => {
                    BackgroundPacingFreshness::Stale
                }
                BackgroundPacingProgressionDrift::RebindRequiredReadinessCounters => {
                    BackgroundPacingFreshness::RebindRequired
                }
            },
            admitted_counters,
            observed_counters,
        })
    }

    pub const fn freshness(self) -> BackgroundPacingFreshness {
        self.freshness
    }

    pub const fn admitted_counters(self) -> IoSchedulerIsolationCounterSnapshot {
        self.admitted_counters
    }

    pub const fn observed_counters(self) -> IoSchedulerIsolationCounterSnapshot {
        self.observed_counters
    }
}
