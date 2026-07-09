use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use crate::runtime::{WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationIndexSupportStatus {
    Verified,
}

impl WorthQueryGraphObligationIndexSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationIndexSupportRow {
    obligation_kind: WorthQueryGraphObligationKind,
    status: WorthQueryGraphObligationIndexSupportStatus,
    lane: WorthQueryGraphObligationSupportLane,
    row_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationIndexSupportRow {
    pub(super) fn verified(obligation_kind: WorthQueryGraphObligationKind) -> Self {
        let status = WorthQueryGraphObligationIndexSupportStatus::Verified;
        let lane = WorthQueryGraphObligationSupportLane::AssemblyIndexSelection;
        let row_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationIndexSupportRow)
                .field_shape(
                    WorthQueryEvidenceTag::new("obligation_kind"),
                    obligation_kind.as_str(),
                )
                .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
                .field_shape(WorthQueryEvidenceTag::new("lane"), lane.as_str())
                .seal();
        Self {
            obligation_kind,
            status,
            lane,
            row_digest,
        }
    }

    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn status(&self) -> WorthQueryGraphObligationIndexSupportStatus {
        self.status
    }

    pub fn lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.lane
    }

    pub fn lane_label(&self) -> &'static str {
        self.lane.as_str()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub(crate) fn row_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.row_digest
    }
}

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn graph_obligation_index_support_rows(
) -> Vec<WorthQueryGraphObligationIndexSupportRow> {
    [
        WorthQueryGraphObligationKind::BlockingInvariant,
        WorthQueryGraphObligationKind::SchemaContractValidator,
        WorthQueryGraphObligationKind::AdvisoryObligation,
        WorthQueryGraphObligationKind::PreflightSequencingObligation,
        WorthQueryGraphObligationKind::CapabilityGapScreen,
        WorthQueryGraphObligationKind::OperatingContextGate,
    ]
    .into_iter()
    .map(WorthQueryGraphObligationIndexSupportRow::verified)
    .collect()
}
