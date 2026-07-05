use crate::declaration::stable_text_digest;
use crate::graph::UiGraphMeasurementNeighborhoodHint;
use crate::obligations::touch::{
    UiGraphTouchAspectFact, UiGraphTouchMeasurementNeighborhoodHint, UiGraphTouchOriginReceipt,
    UiGraphTouchTarget, UiGraphTouchTiming, UiGraphTouchWorld,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTouchDescriptor {
    target: UiGraphTouchTarget,
    origin: UiGraphTouchOriginReceipt,
    world: UiGraphTouchWorld,
    timing: UiGraphTouchTiming,
    aspects: Vec<UiGraphTouchAspectFact>,
    identity_digest: u64,
}

impl UiGraphTouchDescriptor {
    pub(crate) fn new(
        target: UiGraphTouchTarget,
        origin: UiGraphTouchOriginReceipt,
        world: UiGraphTouchWorld,
        timing: UiGraphTouchTiming,
        aspects: Vec<UiGraphTouchAspectFact>,
    ) -> Self {
        let identity_digest = aspects.iter().fold(
            stable_text_digest("graph-touch-descriptor")
                ^ (origin.class() as u64).rotate_left(3)
                ^ origin.authority_digest().rotate_left(7)
                ^ world.world_profile().identity_digest().rotate_left(17)
                ^ (timing as u64).rotate_left(29)
                ^ target.identity_digest().rotate_left(37),
            |digest, fact| {
                digest
                    ^ ((fact.lane() as u64).rotate_left(5)
                        ^ (fact.posture() as u64).rotate_left(19))
            },
        );

        Self {
            target,
            origin,
            world,
            timing,
            aspects,
            identity_digest,
        }
    }

    pub fn target(&self) -> &UiGraphTouchTarget {
        &self.target
    }

    pub fn origin(&self) -> &UiGraphTouchOriginReceipt {
        &self.origin
    }

    pub fn world(&self) -> &UiGraphTouchWorld {
        &self.world
    }

    pub fn timing(&self) -> UiGraphTouchTiming {
        self.timing
    }

    pub fn aspects(&self) -> &[UiGraphTouchAspectFact] {
        &self.aspects
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub fn project_measurement_neighborhood_hint(
        &self,
        neighborhood_hint: &UiGraphMeasurementNeighborhoodHint,
    ) -> Option<UiGraphTouchMeasurementNeighborhoodHint> {
        UiGraphTouchMeasurementNeighborhoodHint::from_touch(self, neighborhood_hint)
    }
}
