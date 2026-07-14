use crate::ordinary::read::{self, WorthQueryReadDeclaration, WorthQueryReadDeclarationStop};
use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonIntent {
    Diff,
    StructuralCorrespondence,
    Lineage,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonDeclaration {
    read: WorthQueryReadDeclaration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonRefinement {
    pub(crate) read: WorthQueryReadDeclaration,
    pub(crate) intent: WorthQueryComparisonIntent,
    pub(crate) candidate_budget: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonDeclarationStop {
    read_stop: WorthQueryReadDeclarationStop,
}

impl WorthQueryComparisonDeclarationStop {
    pub fn denial(&self) -> &WorthQueryReadDenial {
        self.read_stop.denial()
    }

    pub fn next_action(&self) -> super::WorthQueryComparisonNextAction {
        super::WorthQueryComparisonNextAction::ReviseDeclaration
    }
}

impl WorthQueryComparisonDeclaration {
    pub fn diff(self) -> WorthQueryComparisonRefinement {
        self.refine(WorthQueryComparisonIntent::Diff, 0)
    }

    pub fn correspondence(self, candidate_budget: usize) -> WorthQueryComparisonRefinement {
        self.refine(
            WorthQueryComparisonIntent::StructuralCorrespondence,
            candidate_budget.max(1),
        )
    }

    pub fn lineage(self) -> WorthQueryComparisonRefinement {
        self.refine(WorthQueryComparisonIntent::Lineage, 1)
    }

    fn refine(
        self,
        intent: WorthQueryComparisonIntent,
        candidate_budget: usize,
    ) -> WorthQueryComparisonRefinement {
        WorthQueryComparisonRefinement {
            read: self.read,
            intent,
            candidate_budget,
        }
    }
}

impl WorthQueryComparisonRefinement {
    pub fn intent(&self) -> WorthQueryComparisonIntent {
        self.intent
    }

    pub fn candidate_budget(&self) -> usize {
        self.candidate_budget
    }

    pub fn using(
        self,
        context: super::WorthQueryComparisonContext,
    ) -> super::WorthQueryComparisonRequest {
        super::WorthQueryComparisonRequest::new(self, context)
    }
}

pub fn declare(
    author: impl FnOnce(
        WorthQueryReadBuilder<crate::ordinary::read::WorthQueryDeclaredReadIntent>,
    ) -> Result<
        crate::ordinary::read::WorthQueryDeclaredReadIntent,
        WorthQueryReadDenial,
    >,
) -> Result<WorthQueryComparisonDeclaration, WorthQueryComparisonDeclarationStop> {
    read::declare(author)
        .map(|read| WorthQueryComparisonDeclaration { read })
        .map_err(|read_stop| WorthQueryComparisonDeclarationStop { read_stop })
}
