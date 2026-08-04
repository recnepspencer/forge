use crate::domain_computation::authorization::{
    WorthQueryElevationApprovalBinding, WorthQueryElevationCloseBinding,
    WorthQueryElevationRequestBinding, WorthQueryMandatoryReviewBinding,
};

pub(in crate::domain_computation) enum WorthQueryOperationAuthorizationBasis<Input> {
    Conventional,
    Capability {
        input: Input,
    },
    ElevationRequest {
        input: Input,
        binding: WorthQueryElevationRequestBinding,
    },
    ElevationApproval {
        input: Input,
        binding: WorthQueryElevationApprovalBinding,
    },
    ElevationClose {
        input: Input,
        binding: WorthQueryElevationCloseBinding,
    },
    MandatoryReview {
        input: Input,
        binding: WorthQueryMandatoryReviewBinding,
    },
}

impl<Input> WorthQueryOperationAuthorizationBasis<Input> {
    pub(super) const fn capability_input(&self) -> Option<&Input> {
        match self {
            Self::Conventional => None,
            Self::Capability { input }
            | Self::ElevationRequest { input, .. }
            | Self::ElevationApproval { input, .. }
            | Self::ElevationClose { input, .. }
            | Self::MandatoryReview { input, .. } => Some(input),
        }
    }

    pub(super) const fn is_elevation_lifecycle(&self) -> bool {
        matches!(
            self,
            Self::ElevationRequest { .. }
                | Self::ElevationApproval { .. }
                | Self::ElevationClose { .. }
                | Self::MandatoryReview { .. }
        )
    }
}
