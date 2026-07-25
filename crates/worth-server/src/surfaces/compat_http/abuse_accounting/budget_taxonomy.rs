use super::budget_receipt::{WorthServerAbuseBudgetDenialClass, WorthServerTransferByteClass};

impl WorthServerAbuseBudgetDenialClass {
    pub(super) fn into_denial_option(self, detail: String) -> Option<String> {
        match self {
            Self::Admitted => None,
            Self::OrdinaryDenial | Self::SlowlorisCutoff => Some(detail),
        }
    }
}

impl WorthServerTransferByteClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::StructuredPayload => "structured_payload",
            Self::BinaryWire => "binary_wire",
            Self::BinaryAuthoritative => "binary_authoritative",
            Self::MetadataOnly => "metadata_only",
        }
    }
}
