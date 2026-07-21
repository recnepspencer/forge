use super::{WorthUiPlanRegionMutation, WorthUiPlanRegionSuccessor, WorthUiPredecessorRegionProof};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPlanRegionSuccessorDenial {
    CreateCollidesWithPredecessor,
    MissingPredecessorRegion,
    DuplicateMutation,
    HandleCapacity(crate::runtime::WorthUiHandleCapacityExhaustion),
    RegionalStore(super::WorthUiPlanRegionStoreDenial),
}

pub(crate) struct WorthUiPlanRegionSuccessorBuilder;

impl WorthUiPlanRegionSuccessorBuilder {
    pub(crate) fn build(
        proof: WorthUiPredecessorRegionProof,
    ) -> Result<WorthUiPlanRegionSuccessor, WorthUiPlanRegionSuccessorDenial> {
        let predecessor = proof.exact_predecessor().region_store();
        let mut mutated_regions = std::collections::BTreeSet::new();
        for mutation in proof.delta().mutations() {
            if !mutated_regions.insert(mutation.identity().clone()) {
                return Err(WorthUiPlanRegionSuccessorDenial::DuplicateMutation);
            }
            let exists = predecessor.handle_for(mutation.identity()).is_some();
            match mutation {
                WorthUiPlanRegionMutation::Insert(_) if exists => {
                    return Err(WorthUiPlanRegionSuccessorDenial::CreateCollidesWithPredecessor);
                }
                WorthUiPlanRegionMutation::Replace(_)
                | WorthUiPlanRegionMutation::Reparent(_)
                | WorthUiPlanRegionMutation::Rebind(_)
                | WorthUiPlanRegionMutation::LaneTransition(_)
                | WorthUiPlanRegionMutation::Retire(_)
                | WorthUiPlanRegionMutation::RetireOwner(_)
                    if !exists =>
                {
                    return Err(WorthUiPlanRegionSuccessorDenial::MissingPredecessorRegion);
                }
                _ => {}
            }
        }
        let successor = predecessor
            .try_successor(proof.delta().mutations().to_vec())
            .map_err(|denial| match denial {
                super::WorthUiPlanRegionStoreDenial::HandleCapacity(exhaustion) => {
                    WorthUiPlanRegionSuccessorDenial::HandleCapacity(exhaustion)
                }
                other => WorthUiPlanRegionSuccessorDenial::RegionalStore(other),
            })?;
        Ok(successor)
    }
}
