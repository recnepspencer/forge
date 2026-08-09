use std::rc::Rc;

use worth_ui_host_contract::{
    UiMountedPresentationDelta, UiMountedPresentationDeltaInput, UiMountedPresentationInitial,
    UiMountedPresentationInitialInput, UiMountedPresentationUnchanged,
    UiMountedPresentationUnchangedInput, UiMountedPresentationWorkView,
};

use super::lease::UiMountedPresentationLease;

pub(crate) struct UiMountedPresentationWork {
    authority: Rc<()>,
    kind: UiMountedPresentationWorkKind,
}

enum UiMountedPresentationWorkKind {
    Initial(UiMountedPresentationInitial),
    Delta(UiMountedPresentationDelta),
    Unchanged(UiMountedPresentationUnchanged),
}

impl UiMountedPresentationWork {
    pub(crate) fn view(&self) -> UiMountedPresentationWorkView<'_> {
        match &self.kind {
            UiMountedPresentationWorkKind::Initial(initial) => {
                UiMountedPresentationWorkView::Initial(initial)
            }
            UiMountedPresentationWorkKind::Delta(delta) => {
                UiMountedPresentationWorkView::Delta(delta)
            }
            UiMountedPresentationWorkKind::Unchanged(unchanged) => {
                UiMountedPresentationWorkView::Unchanged(unchanged)
            }
        }
    }

    pub(crate) fn issued_by(&self, seal: &Rc<()>) -> bool {
        Rc::ptr_eq(&self.authority, seal)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn into_initial_mechanics(self) -> Option<UiMountedPresentationInitial> {
        match self.kind {
            UiMountedPresentationWorkKind::Initial(initial) => Some(initial),
            UiMountedPresentationWorkKind::Delta(_)
            | UiMountedPresentationWorkKind::Unchanged(_) => None,
        }
    }
}

impl UiMountedPresentationLease {
    pub(crate) fn issue_initial(
        &self,
        input: UiMountedPresentationInitialInput,
    ) -> UiMountedPresentationWork {
        super::validation::validate_initial(&input);
        UiMountedPresentationWork {
            authority: Rc::clone(&self.seal),
            kind: UiMountedPresentationWorkKind::Initial(
                UiMountedPresentationInitial::from_inert_mechanics(input),
            ),
        }
    }

    pub(crate) fn issue_delta(
        &self,
        input: UiMountedPresentationDeltaInput,
    ) -> UiMountedPresentationWork {
        super::validation::validate_delta(&input);
        UiMountedPresentationWork {
            authority: Rc::clone(&self.seal),
            kind: UiMountedPresentationWorkKind::Delta(
                UiMountedPresentationDelta::from_inert_mechanics(input),
            ),
        }
    }

    pub(crate) fn issue_unchanged(
        &self,
        input: UiMountedPresentationUnchangedInput,
    ) -> UiMountedPresentationWork {
        super::validation::validate_unchanged(&input);
        UiMountedPresentationWork {
            authority: Rc::clone(&self.seal),
            kind: UiMountedPresentationWorkKind::Unchanged(
                UiMountedPresentationUnchanged::from_inert_mechanics(input),
            ),
        }
    }
}
