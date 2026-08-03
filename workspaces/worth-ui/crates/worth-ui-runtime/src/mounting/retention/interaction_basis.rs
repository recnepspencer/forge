use worth_ui_host_contract::{UiHostObservationPresentationBasis, UiMountedHitTestMechanic};

use super::UiPresentedFrameBasisRelation;

/// Exact mounting-owned evidence made available to interaction targeting.
pub(crate) struct UiPresentedHitTestBasis {
    presentation: UiHostObservationPresentationBasis,
    relation: UiPresentedFrameBasisRelation,
    rows: Box<[UiMountedHitTestMechanic]>,
}

impl UiPresentedHitTestBasis {
    pub(super) fn new(
        presentation: UiHostObservationPresentationBasis,
        relation: UiPresentedFrameBasisRelation,
        rows: Box<[UiMountedHitTestMechanic]>,
    ) -> Self {
        Self {
            presentation,
            relation,
            rows,
        }
    }

    pub(crate) const fn presentation(&self) -> UiHostObservationPresentationBasis {
        self.presentation
    }

    pub(crate) const fn relation(&self) -> UiPresentedFrameBasisRelation {
        self.relation
    }

    pub(crate) fn rows(&self) -> &[UiMountedHitTestMechanic] {
        &self.rows
    }
}
