use crate::evidence_identity::ForgeQueryEvidenceIdentityScheme;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryJournalPosition, ForgeQueryJournalPositionAuthority};

use super::{ForgeQueryJournalReplayDenial, ForgeQueryJournalReplayDenialKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalSegmentIdentity {
    start_position: u64,
    end_position: u64,
    segment_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryJournalSegmentIdentity {
    pub fn between(
        start: &ForgeQueryJournalPosition,
        end: &ForgeQueryJournalPosition,
    ) -> Result<Self, ForgeQueryJournalReplayDenial> {
        if start.authority() != ForgeQueryJournalPositionAuthority::Committed
            || end.authority() != ForgeQueryJournalPositionAuthority::Committed
        {
            return Err(ForgeQueryJournalReplayDenial::new(
                ForgeQueryJournalReplayDenialKind::InvalidSegmentBounds,
                "journal segment bounds must be committed journal positions",
            ));
        }
        let start_position = start.ordinal_for_reporting();
        let end_position = end.ordinal_for_reporting();
        if start_position > end_position {
            return Err(ForgeQueryJournalReplayDenial::new(
                ForgeQueryJournalReplayDenialKind::InvalidSegmentBounds,
                "journal segment start must not exceed end",
            ));
        }
        Ok(Self::from_committed_bounds(start_position, end_position))
    }

    pub(in crate::runtime) fn from_committed_bounds(
        start_position: u64,
        end_position: u64,
    ) -> Self {
        let segment_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalSegmentIdentity)
                .field_value(
                    ForgeQueryEvidenceTag::new("journal_segment_start"),
                    start_position.to_string(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("journal_segment_end"),
                    end_position.to_string(),
                )
                .seal();
        Self {
            start_position,
            end_position,
            segment_identity,
        }
    }

    pub fn start_position_for_reporting(&self) -> u64 {
        self.start_position
    }

    pub fn end_position_for_reporting(&self) -> u64 {
        self.end_position
    }

    pub fn identity_digest(&self) -> &str {
        self.segment_identity.as_str()
    }

    pub fn identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.segment_identity
    }

    pub fn admit_versioned_committed_bounds_for_replay(
        start_position: u64,
        end_position: u64,
        scheme: ForgeQueryEvidenceIdentityScheme,
    ) -> Result<Self, ForgeQueryJournalReplayDenial> {
        if start_position > end_position {
            return Err(ForgeQueryJournalReplayDenial::new(
                ForgeQueryJournalReplayDenialKind::InvalidSegmentBounds,
                "journal segment start must not exceed end",
            ));
        }
        let segment_identity = ForgeQueryEvidenceIdentity::compose_with_scheme(
            ForgeQueryEvidenceScope::JournalSegmentIdentity,
            scheme,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("journal_segment_start"),
            start_position.to_string(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("journal_segment_end"),
            end_position.to_string(),
        )
        .seal();
        Ok(Self {
            start_position,
            end_position,
            segment_identity,
        })
    }
}
