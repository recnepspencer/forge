use std::sync::Arc;

use super::WorthServerProductSession;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductSessionTerminationKind {
    Expired,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductSessionTermination {
    session: WorthServerProductSession,
    kind: WorthServerProductSessionTerminationKind,
}

impl WorthServerProductSessionTermination {
    pub(crate) fn new(
        session: WorthServerProductSession,
        kind: WorthServerProductSessionTerminationKind,
    ) -> Self {
        Self { session, kind }
    }

    pub fn session(&self) -> &WorthServerProductSession {
        &self.session
    }

    pub fn kind(&self) -> WorthServerProductSessionTerminationKind {
        self.kind
    }
}

pub trait WorthServerProductSessionTerminationObserver: std::fmt::Debug + Send + Sync {
    fn observe_termination(&self, termination: &WorthServerProductSessionTermination);
}

pub(crate) type SharedProductSessionTerminationObserver =
    Arc<dyn WorthServerProductSessionTerminationObserver>;
