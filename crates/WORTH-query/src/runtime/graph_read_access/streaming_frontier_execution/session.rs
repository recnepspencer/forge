use std::collections::BTreeSet;

use super::{
    WorthQueryGraphReadFrontierCursor, WorthQueryGraphReadStreamingCursorDenial,
    WorthQueryGraphReadStreamingCursorDenialKind, WorthQueryGraphReadStreamingPageReceipt,
    WorthQueryGraphReadStreamingReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadStreamingCursorSession {
    streaming_plan_digest: String,
    snapshot_identity_digest: String,
    expected_next_page_ordinal: usize,
    page_receipts: Vec<WorthQueryGraphReadStreamingPageReceipt>,
    consumed_cursor_digests: BTreeSet<String>,
}

impl WorthQueryGraphReadStreamingCursorSession {
    pub fn streaming_plan_digest(&self) -> &str {
        &self.streaming_plan_digest
    }

    pub fn snapshot_identity_digest(&self) -> &str {
        &self.snapshot_identity_digest
    }

    pub fn expected_next_page_ordinal(&self) -> usize {
        self.expected_next_page_ordinal
    }

    pub fn consumed_cursor_count(&self) -> usize {
        self.consumed_cursor_digests.len()
    }

    pub fn resume(
        &mut self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> Result<WorthQueryGraphReadStreamingPageReceipt, WorthQueryGraphReadStreamingCursorDenial>
    {
        self.verify_cursor_plan(cursor)?;
        self.verify_cursor_basis(cursor)?;
        self.verify_cursor_has_not_been_consumed(cursor)?;
        self.verify_cursor_sequence(cursor)?;
        self.verify_cursor_continuation(cursor)?;

        let Some(page) = self
            .page_receipts
            .iter()
            .find(|page| page.page_ordinal() == cursor.next_page_ordinal())
            .cloned()
        else {
            return Err(self.denial(
                WorthQueryGraphReadStreamingCursorDenialKind::CursorSequenceSkipped,
                Some(cursor),
                Some(self.expected_next_page_ordinal.to_string()),
                Some(cursor.next_page_ordinal().to_string()),
            ));
        };
        self.consumed_cursor_digests
            .insert(cursor.digest().to_string());
        self.expected_next_page_ordinal = cursor.next_page_ordinal() + 1;
        Ok(page)
    }

    pub(crate) fn from_receipt(receipt: &WorthQueryGraphReadStreamingReceipt) -> Self {
        Self {
            streaming_plan_digest: receipt.streaming_plan_digest().to_string(),
            snapshot_identity_digest: receipt.snapshot_identity_digest().to_string(),
            expected_next_page_ordinal: 1,
            page_receipts: receipt.page_receipts().to_vec(),
            consumed_cursor_digests: BTreeSet::new(),
        }
    }

    fn verify_cursor_plan(
        &self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> Result<(), WorthQueryGraphReadStreamingCursorDenial> {
        if cursor.streaming_plan_digest() == self.streaming_plan_digest {
            return Ok(());
        }
        Err(self.denial(
            WorthQueryGraphReadStreamingCursorDenialKind::CursorPlanMismatch,
            Some(cursor),
            Some(self.streaming_plan_digest.clone()),
            Some(cursor.streaming_plan_digest().to_string()),
        ))
    }

    fn verify_cursor_basis(
        &self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> Result<(), WorthQueryGraphReadStreamingCursorDenial> {
        if cursor.snapshot_identity_digest() == self.snapshot_identity_digest {
            return Ok(());
        }
        Err(self.denial(
            WorthQueryGraphReadStreamingCursorDenialKind::CursorBasisMismatch,
            Some(cursor),
            Some(self.snapshot_identity_digest.clone()),
            Some(cursor.snapshot_identity_digest().to_string()),
        ))
    }

    fn verify_cursor_has_not_been_consumed(
        &self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> Result<(), WorthQueryGraphReadStreamingCursorDenial> {
        if !self.consumed_cursor_digests.contains(cursor.digest()) {
            return Ok(());
        }
        Err(self.denial(
            WorthQueryGraphReadStreamingCursorDenialKind::CursorReplayDenied,
            Some(cursor),
            None,
            None,
        ))
    }

    fn verify_cursor_sequence(
        &self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> Result<(), WorthQueryGraphReadStreamingCursorDenial> {
        if cursor.next_page_ordinal() == self.expected_next_page_ordinal {
            return Ok(());
        }
        Err(self.denial(
            WorthQueryGraphReadStreamingCursorDenialKind::CursorSequenceSkipped,
            Some(cursor),
            Some(self.expected_next_page_ordinal.to_string()),
            Some(cursor.next_page_ordinal().to_string()),
        ))
    }

    fn verify_cursor_continuation(
        &self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> Result<(), WorthQueryGraphReadStreamingCursorDenial> {
        let prior_page_ordinal = cursor.next_page_ordinal().saturating_sub(1);
        let Some(prior_page) = self
            .page_receipts
            .iter()
            .find(|page| page.page_ordinal() == prior_page_ordinal)
        else {
            return Err(self.worthd_cursor_denial(cursor));
        };
        let Some(prior_cursor) = prior_page.next_cursor() else {
            return Err(self.worthd_cursor_denial(cursor));
        };
        if cursor.prior_page_receipt_digest() == prior_page.digest()
            && cursor.digest() == prior_cursor.digest()
        {
            return Ok(());
        }
        Err(self.worthd_cursor_denial(cursor))
    }

    fn worthd_cursor_denial(
        &self,
        cursor: &WorthQueryGraphReadFrontierCursor,
    ) -> WorthQueryGraphReadStreamingCursorDenial {
        self.denial(
            WorthQueryGraphReadStreamingCursorDenialKind::WorthdCursorDenied,
            Some(cursor),
            None,
            None,
        )
    }

    fn denial(
        &self,
        kind: WorthQueryGraphReadStreamingCursorDenialKind,
        cursor: Option<&WorthQueryGraphReadFrontierCursor>,
        expected_identity_digest: Option<String>,
        observed_identity_digest: Option<String>,
    ) -> WorthQueryGraphReadStreamingCursorDenial {
        WorthQueryGraphReadStreamingCursorDenial::new(
            kind,
            self.streaming_plan_digest.clone(),
            cursor.map(|cursor| cursor.digest().to_string()),
            expected_identity_digest,
            observed_identity_digest,
        )
    }
}
