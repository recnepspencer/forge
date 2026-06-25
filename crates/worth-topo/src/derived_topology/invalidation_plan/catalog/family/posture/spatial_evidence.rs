use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologySpatialEvidencePosture {
    NoSpatialEvidenceConsumed,
    SpatialReceiptRequired,
}

impl DerivedTopologySpatialEvidencePosture {
    pub const fn requires_spatial_receipt(self) -> bool {
        matches!(self, Self::SpatialReceiptRequired)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSpatialEvidenceConsumed => "no_spatial_evidence_consumed",
            Self::SpatialReceiptRequired => "spatial_receipt_required",
        }
    }
}
