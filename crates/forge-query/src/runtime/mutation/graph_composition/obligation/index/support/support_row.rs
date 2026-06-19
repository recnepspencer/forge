use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use crate::runtime::{ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationIndexSupportStatus {
    Verified,
}

impl ForgeQueryGraphObligationIndexSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationIndexSupportRow {
    obligation_kind: ForgeQueryGraphObligationKind,
    status: ForgeQueryGraphObligationIndexSupportStatus,
    lane: ForgeQueryGraphObligationSupportLane,
    row_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationIndexSupportRow {
    pub(super) fn verified(obligation_kind: ForgeQueryGraphObligationKind) -> Self {
        let status = ForgeQueryGraphObligationIndexSupportStatus::Verified;
        let lane = ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection;
        let row_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationIndexSupportRow)
                .field_shape(
                    ForgeQueryEvidenceTag::new("obligation_kind"),
                    obligation_kind.as_str(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
                .field_shape(ForgeQueryEvidenceTag::new("lane"), lane.as_str())
                .seal();
        Self {
            obligation_kind,
            status,
            lane,
            row_digest,
        }
    }

    pub fn obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn status(&self) -> ForgeQueryGraphObligationIndexSupportStatus {
        self.status
    }

    pub fn lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.lane
    }

    pub fn lane_label(&self) -> &'static str {
        self.lane.as_str()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub(crate) fn row_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_digest
    }
}

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn graph_obligation_index_support_rows(
) -> Vec<ForgeQueryGraphObligationIndexSupportRow> {
    [
        ForgeQueryGraphObligationKind::BlockingInvariant,
        ForgeQueryGraphObligationKind::SchemaContractValidator,
        ForgeQueryGraphObligationKind::AdvisoryObligation,
        ForgeQueryGraphObligationKind::PreflightSequencingObligation,
        ForgeQueryGraphObligationKind::CapabilityGapScreen,
        ForgeQueryGraphObligationKind::OperatingContextGate,
    ]
    .into_iter()
    .map(ForgeQueryGraphObligationIndexSupportRow::verified)
    .collect()
}
