use crate::{PageRedoDigestState, PageRedoEligibility};
use worth_store_physical_format::PhysicalPageId;

use super::{AdmittedRedoFrame, RedoPlanningDenial, RedoPlanningDenialKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoApplicationCursor {
    pages: Vec<RedoApplicationPageFact>,
}

impl RedoApplicationCursor {
    pub fn new(mut pages: Vec<RedoApplicationPageFact>) -> Result<Self, RedoPlanningDenial> {
        for page in &pages {
            page.require_page_generation_coherence()?;
        }
        pages.sort_by_key(|page| page.page_id());
        Ok(Self { pages })
    }

    pub fn pages(&self) -> &[RedoApplicationPageFact] {
        &self.pages
    }

    pub(crate) fn apply_frame(
        &mut self,
        frame: &AdmittedRedoFrame,
    ) -> Result<bool, RedoPlanningDenial> {
        let Some(page) = self
            .pages
            .iter_mut()
            .find(|page| page.page_id() == frame.target_page())
        else {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::MissingPageEligibility {
                    frame_lsn: frame.redo_lsn(),
                },
            ));
        };
        let before = page.digest_state.clone();
        let after = page
            .eligibility
            .apply_idempotent_redo(before.clone(), frame.application_basis())
            .map_err(|denial| {
                RedoPlanningDenial::new(RedoPlanningDenialKind::PageRedoDenied {
                    frame_lsn: frame.redo_lsn(),
                    denial,
                })
            })?;
        let skipped = after == before;
        page.digest_state = after;
        Ok(skipped)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoApplicationPageFact {
    page_id: PhysicalPageId,
    eligibility: PageRedoEligibility,
    digest_state: PageRedoDigestState,
}

impl RedoApplicationPageFact {
    pub fn new(
        page_id: PhysicalPageId,
        eligibility: PageRedoEligibility,
        digest_state: PageRedoDigestState,
    ) -> Self {
        Self {
            page_id,
            eligibility,
            digest_state,
        }
    }

    pub const fn page_id(&self) -> PhysicalPageId {
        self.page_id
    }

    pub const fn eligibility(&self) -> &PageRedoEligibility {
        &self.eligibility
    }

    pub const fn digest_state(&self) -> &PageRedoDigestState {
        &self.digest_state
    }

    fn require_page_generation_coherence(&self) -> Result<(), RedoPlanningDenial> {
        let eligibility_page = self.eligibility.page_generation().page_id();
        let digest_page = self.digest_state.page_generation().page_id();
        if self.page_id == eligibility_page && self.page_id == digest_page {
            return Ok(());
        }
        Err(RedoPlanningDenial::new(
            RedoPlanningDenialKind::CursorPageGenerationMismatch {
                cursor_page: self.page_id,
                eligibility_page,
                digest_page,
            },
        ))
    }
}
