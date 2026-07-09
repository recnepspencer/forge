use crate::{LogSequenceNumber, PageLsn, PageRedoApplicationBasis};
use worth_store_physical_format::{PageGenerationCell, PhysicalPageId};

use super::{RedoPlanningDenial, RedoPlanningDenialKind, RedoRecordGrammar, WalValidPrefix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedRedoFrame {
    target_page: PhysicalPageId,
    target_generation: PageGenerationCell,
    redo_lsn: LogSequenceNumber,
    page_lsn_basis: PageLsn,
    application_basis: PageRedoApplicationBasis,
}

impl AdmittedRedoFrame {
    pub fn admit(
        grammar: RedoRecordGrammar,
        valid_prefix: &WalValidPrefix,
    ) -> Result<Self, RedoPlanningDenial> {
        if !valid_prefix.contains_lsn(grammar.redo_lsn()) {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::FrameOutsideAdmittedSourceRange {
                    frame_lsn: grammar.redo_lsn(),
                    source_range: valid_prefix.prefix_range(),
                },
            ));
        }
        let redo_page_lsn = PageLsn::from_lsn(grammar.redo_lsn());
        if grammar.page_lsn_basis() != redo_page_lsn {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::WrongPageLsnBasis {
                    frame_lsn: grammar.redo_lsn(),
                    page_lsn_basis: grammar.page_lsn_basis(),
                },
            ));
        }
        let target_generation = grammar.target_generation().generation();
        if grammar.target_page() != target_generation.page_id() {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::RedoTargetPageGenerationMismatch {
                    target_page: grammar.target_page(),
                    generation_page: target_generation.page_id(),
                },
            ));
        }
        let application_basis = PageRedoApplicationBasis::new(
            target_generation,
            redo_page_lsn,
            grammar.operation_form().digest(),
            grammar.idempotence_basis().digest(),
        );
        Ok(Self {
            target_page: grammar.target_page(),
            target_generation,
            redo_lsn: grammar.redo_lsn(),
            page_lsn_basis: grammar.page_lsn_basis(),
            application_basis,
        })
    }

    pub const fn target_page(&self) -> PhysicalPageId {
        self.target_page
    }

    pub const fn target_generation(&self) -> PageGenerationCell {
        self.target_generation
    }

    pub const fn redo_lsn(&self) -> LogSequenceNumber {
        self.redo_lsn
    }

    pub const fn page_lsn_basis(&self) -> PageLsn {
        self.page_lsn_basis
    }

    pub const fn application_basis(&self) -> &PageRedoApplicationBasis {
        &self.application_basis
    }
}
