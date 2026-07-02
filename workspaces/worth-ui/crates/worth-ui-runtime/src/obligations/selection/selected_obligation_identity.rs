use crate::declaration::stable_text_digest;
use crate::graph::UiGraphWorldProfile;
use crate::obligations::catalog::UiObligationFamily;
use crate::obligations::touch::{UiGraphTouchRuntimeLane, UiGraphTouchTarget, UiGraphTouchWorld};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationSupportBasis {
    TouchMeaning,
    QueryBinding,
    ServiceUsage,
    MeasurementPolicy,
    HostCapability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSelectedObligationIdentity {
    touch_identity_digest: u64,
    obligation_family: UiObligationFamily,
    target: UiGraphTouchTarget,
    aspect_scope: Box<[UiGraphTouchRuntimeLane]>,
    world: UiGraphTouchWorld,
    support_basis: UiObligationSupportBasis,
    identity_digest: u64,
}

impl UiSelectedObligationIdentity {
    pub(crate) fn new(
        touch_identity_digest: u64,
        obligation_family: UiObligationFamily,
        target: &UiGraphTouchTarget,
        aspect_scope: Box<[UiGraphTouchRuntimeLane]>,
        world: &UiGraphTouchWorld,
        support_basis: UiObligationSupportBasis,
    ) -> Self {
        let identity_digest = aspect_scope.iter().fold(
            stable_text_digest("selected-obligation-identity")
                ^ touch_identity_digest.rotate_left(7)
                ^ (obligation_family as u64).rotate_left(13)
                ^ target.identity_digest().rotate_left(19)
                ^ world.world_profile().identity_digest().rotate_left(29)
                ^ (support_basis as u64).rotate_left(37),
            |digest, lane| digest ^ (*lane as u64).rotate_left(11),
        );

        Self {
            touch_identity_digest,
            obligation_family,
            target: target.clone(),
            aspect_scope,
            world: world.clone(),
            support_basis,
            identity_digest,
        }
    }

    pub fn touch_identity_digest(&self) -> u64 {
        self.touch_identity_digest
    }

    pub fn obligation_family(&self) -> UiObligationFamily {
        self.obligation_family
    }

    pub fn target(&self) -> &UiGraphTouchTarget {
        &self.target
    }

    pub fn aspect_scope(&self) -> &[UiGraphTouchRuntimeLane] {
        &self.aspect_scope
    }

    pub fn world(&self) -> &UiGraphTouchWorld {
        &self.world
    }

    pub fn world_profile(&self) -> &UiGraphWorldProfile {
        self.world.world_profile()
    }

    pub fn support_basis(&self) -> UiObligationSupportBasis {
        self.support_basis
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
