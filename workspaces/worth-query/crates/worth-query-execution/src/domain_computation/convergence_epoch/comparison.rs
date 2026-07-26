use std::sync::Arc;

use super::identity_validation::portable_identity;
use super::{
    WorthQueryConvergenceDisposition, WorthQueryConvergenceFeasibility,
    WorthQueryConvergenceIncumbentUpdate,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConvergenceComparison {
    candidate_occurrence_identity: Arc<str>,
    state_identity: Arc<str>,
    disposition: WorthQueryConvergenceDisposition,
    feasibility: WorthQueryConvergenceFeasibility,
    incumbent_update: WorthQueryConvergenceIncumbentUpdate,
}

impl WorthQueryConvergenceComparison {
    pub fn new(
        candidate_occurrence_identity: impl Into<Arc<str>>,
        state_identity: impl Into<Arc<str>>,
        disposition: WorthQueryConvergenceDisposition,
        feasibility: WorthQueryConvergenceFeasibility,
        incumbent_update: WorthQueryConvergenceIncumbentUpdate,
    ) -> Result<Self, &'static str> {
        let candidate_occurrence_identity = candidate_occurrence_identity.into();
        let state_identity = state_identity.into();
        if !portable_identity(&candidate_occurrence_identity) || !portable_identity(&state_identity)
        {
            return Err("invalid-convergence-comparison-identity");
        }
        validate_incumbent_removals(&incumbent_update)?;
        Ok(Self {
            candidate_occurrence_identity,
            state_identity,
            disposition,
            feasibility,
            incumbent_update,
        })
    }

    pub fn candidate_occurrence_identity(&self) -> &str {
        &self.candidate_occurrence_identity
    }

    pub fn state_identity(&self) -> &str {
        &self.state_identity
    }

    pub const fn disposition(&self) -> WorthQueryConvergenceDisposition {
        self.disposition
    }

    pub const fn feasibility(&self) -> WorthQueryConvergenceFeasibility {
        self.feasibility
    }

    pub fn incumbent_update(&self) -> &WorthQueryConvergenceIncumbentUpdate {
        &self.incumbent_update
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<str>,
        Arc<str>,
        WorthQueryConvergenceDisposition,
        WorthQueryConvergenceFeasibility,
        WorthQueryConvergenceIncumbentUpdate,
    ) {
        (
            self.candidate_occurrence_identity,
            self.state_identity,
            self.disposition,
            self.feasibility,
            self.incumbent_update,
        )
    }
}

fn validate_incumbent_removals(
    incumbent_update: &WorthQueryConvergenceIncumbentUpdate,
) -> Result<(), &'static str> {
    let WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd {
        removed_occurrence_identities,
    } = incumbent_update
    else {
        return Ok(());
    };
    let mut normalized = removed_occurrence_identities
        .iter()
        .map(|identity| identity.as_ref())
        .collect::<Vec<_>>();
    if normalized.is_empty()
        || normalized
            .iter()
            .any(|identity| !portable_identity(identity))
    {
        return Err("invalid-convergence-incumbent-removal");
    }
    normalized.sort_unstable();
    if normalized.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("duplicate-convergence-incumbent-removal");
    }
    Ok(())
}
