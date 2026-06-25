use crate::graph_read_access_inventory::{
    WorthGraphReadAccessMilestoneSevenSeed, WorthGraphReadDeclarationCandidate,
    WorthGraphReadDeletionLedgerItem, WorthGraphReadQueryAccessCapabilityGap,
};

use super::super::phase_one_closeout::{
    WorthGraphReadAccessDeclarationPhaseOneError, WorthGraphReadAccessDeclarationPhaseOneErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessDeclarationAdmittedSeed {
    seed: WorthGraphReadAccessMilestoneSevenSeed,
}

pub(crate) fn admit_milestone_seven_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> Result<WorthGraphReadAccessDeclarationAdmittedSeed, WorthGraphReadAccessDeclarationPhaseOneError>
{
    if seed.claims_execution_authority() {
        return Err(error(
            WorthGraphReadAccessDeclarationPhaseOneErrorKind::SeedClaimedExecutionAuthority,
        ));
    }
    if seed.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap() {
        return Err(error(
            WorthGraphReadAccessDeclarationPhaseOneErrorKind::SeedContainsUncappedOldGraphReadFolklore,
        ));
    }
    Ok(WorthGraphReadAccessDeclarationAdmittedSeed { seed: seed.clone() })
}

impl WorthGraphReadAccessDeclarationAdmittedSeed {
    pub(crate) const fn milestone_seven_seed(&self) -> &WorthGraphReadAccessMilestoneSevenSeed {
        &self.seed
    }

    pub(crate) fn declaration_candidates(&self) -> &[WorthGraphReadDeclarationCandidate] {
        self.seed.declaration_candidates()
    }

    pub(crate) fn capability_gaps(&self) -> &[WorthGraphReadQueryAccessCapabilityGap] {
        self.seed.capability_gaps()
    }

    pub(crate) fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        self.seed.deletion_items()
    }
}

const fn error(
    kind: WorthGraphReadAccessDeclarationPhaseOneErrorKind,
) -> WorthGraphReadAccessDeclarationPhaseOneError {
    WorthGraphReadAccessDeclarationPhaseOneError::new(kind)
}
