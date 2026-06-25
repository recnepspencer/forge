use std::collections::BTreeSet;

use super::structured_seed_pairing::WorthGraphReadAccessPlanAdoptionSeedPairing;
use crate::graph_read_access_plan_adoption::phase_two_adoption::errors::{
    WorthGraphReadAccessPlanAdoptionPhaseTwoError,
    WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind,
};

pub(crate) fn reject_duplicate_pairings(
    pairings: &[WorthGraphReadAccessPlanAdoptionSeedPairing],
) -> Result<(), WorthGraphReadAccessPlanAdoptionPhaseTwoError> {
    let mut pairing_digests = BTreeSet::new();
    let mut row_pairs = BTreeSet::new();

    for pairing in pairings {
        if !pairing_digests.insert(pairing.pairing_digest()) {
            return Err(error(
                WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::DuplicateStructuredSeedPairing,
            ));
        }

        if !row_pairs.insert((
            pairing.read_family_identity_digest(),
            pairing.requirement_row_digest(),
        )) {
            return Err(error(
                WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::DuplicateStructuredSeedPairing,
            ));
        }
    }

    Ok(())
}

const fn error(
    kind: WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind,
) -> WorthGraphReadAccessPlanAdoptionPhaseTwoError {
    WorthGraphReadAccessPlanAdoptionPhaseTwoError::new(kind)
}
