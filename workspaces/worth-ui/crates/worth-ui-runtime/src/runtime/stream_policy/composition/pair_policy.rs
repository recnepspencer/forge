use super::policy_join::{join_contract_policies, resolved_family_policy};
use super::{
    UiAllocationFamilyPairOutcome, UiAllocationStreamCompositionDenial, UiAllocationStreamFamily,
    UiResolvedAllocationStreamPolicy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiAllocationFamilyPairContract {
    left: UiAllocationStreamFamily,
    right: UiAllocationStreamFamily,
    outcome: UiAllocationFamilyPairOutcome,
    resolved: UiResolvedAllocationStreamPolicy,
}

pub(super) fn pair_contract(
    left: UiAllocationStreamFamily,
    right: UiAllocationStreamFamily,
) -> Result<UiAllocationFamilyPairContract, UiAllocationStreamCompositionDenial> {
    use UiAllocationFamilyPairOutcome::{CoSelect, Compose, Deny};
    let pair = if left.canonical_order() <= right.canonical_order() {
        (left, right)
    } else {
        (right, left)
    };
    let outcome = match pair {
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::DurableResize) => Deny,
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::ViewportObservation)
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::PortalAnchorObservation,
        ) => Deny,
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::ResizePreview)
        | (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::ResizePreview)
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::ResizePreview)
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::PortalAnchorObservation,
        ) => CoSelect,
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::TextInput)
        | (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::DurableResize)
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::DurableResize)
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::ResizePreview)
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::PortalAnchorObservation,
        ) => Compose,
        (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::TextInput)
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::TextInput,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (UiAllocationStreamFamily::ViewportObservation, UiAllocationStreamFamily::TextInput)
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::TextInput)
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::TextInput)
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::DurableResize)
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::TextInput,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::TextInput,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::ScrollExtentObservation,
        ) => {
            unreachable!("pair normalization must produce canonical family order")
        }
    };
    if outcome == Deny {
        return Err(UiAllocationStreamCompositionDenial::IllegalFamilyPair {
            left: pair.0,
            right: pair.1,
        });
    }
    Ok(UiAllocationFamilyPairContract {
        left: pair.0,
        right: pair.1,
        outcome,
        resolved: join_contract_policies(
            resolved_family_policy(pair.0),
            resolved_family_policy(pair.1),
        ),
    })
}

impl UiAllocationFamilyPairContract {
    pub(super) fn left(self) -> UiAllocationStreamFamily {
        self.left
    }
    pub(super) fn right(self) -> UiAllocationStreamFamily {
        self.right
    }
    pub(super) fn outcome(self) -> UiAllocationFamilyPairOutcome {
        self.outcome
    }
    pub(super) fn resolved(self) -> UiResolvedAllocationStreamPolicy {
        self.resolved
    }
}
