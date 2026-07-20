use crate::ordinary::read::{self, WorthQueryReadDeclaration, WorthQueryReadDeclarationStop};
use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial};

use super::WorthQueryHistoricalContext;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryHistoricalPathKind {
    RetainedSnapshot,
    DeltaReplay,
    FullReconstruction,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryHistoricalDeclaration {
    read: WorthQueryReadDeclaration,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryHistoricalPathDeclaration {
    pub(crate) read: WorthQueryReadDeclaration,
    pub(crate) path: WorthQueryHistoricalPathKind,
    pub(crate) replay_budget: usize,
    pub(crate) reconstruction_budget: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryHistoricalDeclarationStop {
    read_stop: WorthQueryReadDeclarationStop,
}

impl WorthQueryHistoricalDeclarationStop {
    pub fn denial(&self) -> &WorthQueryReadDenial {
        self.read_stop.denial()
    }

    pub fn next_action(&self) -> super::WorthQueryHistoricalNextAction {
        super::WorthQueryHistoricalNextAction::ReviseDeclaration
    }
}

impl WorthQueryHistoricalDeclaration {
    pub fn retained_snapshot(self) -> WorthQueryHistoricalPathDeclaration {
        self.path(WorthQueryHistoricalPathKind::RetainedSnapshot, 0, 0)
    }

    pub fn delta_replay(self, replay_budget: usize) -> WorthQueryHistoricalPathDeclaration {
        self.path(WorthQueryHistoricalPathKind::DeltaReplay, replay_budget, 0)
    }

    pub fn full_reconstruction(
        self,
        reconstruction_budget: usize,
    ) -> WorthQueryHistoricalPathDeclaration {
        self.path(
            WorthQueryHistoricalPathKind::FullReconstruction,
            0,
            reconstruction_budget,
        )
    }

    fn path(
        self,
        path: WorthQueryHistoricalPathKind,
        replay_budget: usize,
        reconstruction_budget: usize,
    ) -> WorthQueryHistoricalPathDeclaration {
        WorthQueryHistoricalPathDeclaration {
            read: self.read,
            path,
            replay_budget,
            reconstruction_budget,
        }
    }
}

impl WorthQueryHistoricalPathDeclaration {
    pub(crate) fn retained_from_read(read: WorthQueryReadDeclaration) -> Self {
        Self {
            read,
            path: WorthQueryHistoricalPathKind::RetainedSnapshot,
            replay_budget: 0,
            reconstruction_budget: 0,
        }
    }

    pub fn path_kind(&self) -> WorthQueryHistoricalPathKind {
        self.path
    }

    pub fn replay_budget(&self) -> usize {
        self.replay_budget
    }

    pub fn reconstruction_budget(&self) -> usize {
        self.reconstruction_budget
    }

    pub fn using(self, context: WorthQueryHistoricalContext) -> super::WorthQueryHistoricalRequest {
        super::WorthQueryHistoricalRequest::new(self, context)
    }
}

pub fn declare(
    author: impl FnOnce(
        WorthQueryReadBuilder<crate::ordinary::read::WorthQueryDeclaredReadIntent>,
    ) -> Result<
        crate::ordinary::read::WorthQueryDeclaredReadIntent,
        WorthQueryReadDenial,
    >,
) -> Result<WorthQueryHistoricalDeclaration, WorthQueryHistoricalDeclarationStop> {
    read::declare(author)
        .map(|read| WorthQueryHistoricalDeclaration { read })
        .map_err(|read_stop| WorthQueryHistoricalDeclarationStop { read_stop })
}
