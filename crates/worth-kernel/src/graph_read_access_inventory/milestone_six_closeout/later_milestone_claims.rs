use super::super::phase_six_closeout::WorthGraphReadAccessPhaseSixCloseout;
use super::errors::{
    WorthGraphReadAccessMilestoneSixError, WorthGraphReadAccessMilestoneSixErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessLaterMilestoneClaims {
    query_declarations_complete: bool,
    admitted_access_plans_complete: bool,
    graph_read_receipts_complete: bool,
    validator_derivation_complete: bool,
    invalidation_complete: bool,
    replay_complete: bool,
    conflict_complete: bool,
    cache_complete: bool,
    public_diagnostics_complete: bool,
}

impl WorthGraphReadAccessLaterMilestoneClaims {
    pub(crate) const fn absent() -> Self {
        Self {
            query_declarations_complete: false,
            admitted_access_plans_complete: false,
            graph_read_receipts_complete: false,
            validator_derivation_complete: false,
            invalidation_complete: false,
            replay_complete: false,
            conflict_complete: false,
            cache_complete: false,
            public_diagnostics_complete: false,
        }
    }

    #[cfg(test)]
    pub(crate) const fn with_query_declarations_complete() -> Self {
        Self {
            query_declarations_complete: true,
            ..Self::absent()
        }
    }

    pub(crate) const fn rejects_later_milestone_completion(&self) -> bool {
        self.query_declarations_complete
            || self.admitted_access_plans_complete
            || self.graph_read_receipts_complete
            || self.validator_derivation_complete
            || self.invalidation_complete
            || self.replay_complete
            || self.conflict_complete
            || self.cache_complete
            || self.public_diagnostics_complete
    }
}

pub(crate) fn reject_later_milestone_claims(
    disposition: &WorthGraphReadAccessPhaseSixCloseout,
    claims: WorthGraphReadAccessLaterMilestoneClaims,
) -> Result<(), WorthGraphReadAccessMilestoneSixError> {
    if disposition.claims_execution_authority()
        || disposition.claims_later_milestone_completion()
        || claims.rejects_later_milestone_completion()
    {
        return Err(WorthGraphReadAccessMilestoneSixError::new(
            WorthGraphReadAccessMilestoneSixErrorKind::LaterMilestoneClaimed,
        ));
    }
    Ok(())
}
