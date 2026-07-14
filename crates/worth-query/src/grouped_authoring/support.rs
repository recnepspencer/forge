use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::authoring::AspectFieldKey;
use worth_foundational::facade::{AspectKey, FieldKey};

use super::artifact::WorthQueryGroupedDeclarationArtifact;
use super::posture::{
    WorthQueryGroupedContinuityAssumption, WorthQueryGroupedIntent,
    WorthQueryGroupedSharedPostureClaim,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedSupportFeature {
    GroupedDeclaration,
    GroupedRoute,
    GroupedReceipt,
    GroupedEnvelope,
    GroupedContributionComposition,
    Atomicity,
    GroupingIntent,
    ContinuityAssumption,
    SharedPostureClaims,
}

impl WorthQueryGroupedSupportFeature {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GroupedDeclaration => "grouped_declaration",
            Self::GroupedRoute => "grouped_route",
            Self::GroupedReceipt => "grouped_receipt",
            Self::GroupedEnvelope => "grouped_envelope",
            Self::GroupedContributionComposition => "grouped_contribution_composition",
            Self::Atomicity => "atomicity",
            Self::GroupingIntent => "grouping_intent",
            Self::ContinuityAssumption => "continuity_assumption",
            Self::SharedPostureClaims => "shared_posture_claims",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedSupportStatus {
    Supported,
    Unsupported,
}

impl WorthQueryGroupedSupportStatus {
    pub fn is_supported(self) -> bool {
        self == Self::Supported
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGroupedSupportReport {
    statuses: Vec<(
        WorthQueryGroupedSupportFeature,
        WorthQueryGroupedSupportStatus,
    )>,
    unsupported_claims: Vec<WorthQueryGroupedSharedPostureClaim>,
}

impl WorthQueryGroupedSupportReport {
    fn new(
        statuses: Vec<(
            WorthQueryGroupedSupportFeature,
            WorthQueryGroupedSupportStatus,
        )>,
        unsupported_claims: Vec<WorthQueryGroupedSharedPostureClaim>,
    ) -> Self {
        Self {
            statuses,
            unsupported_claims,
        }
    }

    pub fn statuses(
        &self,
    ) -> &[(
        WorthQueryGroupedSupportFeature,
        WorthQueryGroupedSupportStatus,
    )] {
        &self.statuses
    }

    pub fn status_for(
        &self,
        feature: WorthQueryGroupedSupportFeature,
    ) -> WorthQueryGroupedSupportStatus {
        self.statuses
            .iter()
            .find(|(candidate, _)| *candidate == feature)
            .map(|(_, status)| *status)
            .unwrap_or(WorthQueryGroupedSupportStatus::Unsupported)
    }

    pub fn unsupported_claims(&self) -> &[WorthQueryGroupedSharedPostureClaim] {
        &self.unsupported_claims
    }
}

pub(crate) fn worth_query_grouped_support_report<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    declaration: &WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedSupportReport {
    let unsupported_claims = declaration
        .shared_posture_claims()
        .iter()
        .copied()
        .filter(|claim| !claim_supported(declaration, *claim))
        .collect::<Vec<_>>();
    let shared_status = if unsupported_claims.is_empty() {
        WorthQueryGroupedSupportStatus::Supported
    } else {
        WorthQueryGroupedSupportStatus::Unsupported
    };
    WorthQueryGroupedSupportReport::new(
        vec![
            (
                WorthQueryGroupedSupportFeature::GroupedDeclaration,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::GroupedRoute,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::GroupedReceipt,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::GroupedEnvelope,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::GroupedContributionComposition,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::Atomicity,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::GroupingIntent,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::ContinuityAssumption,
                WorthQueryGroupedSupportStatus::Supported,
            ),
            (
                WorthQueryGroupedSupportFeature::SharedPostureClaims,
                shared_status,
            ),
        ],
        unsupported_claims,
    )
}

fn claim_supported<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    declaration: &WorthQueryGroupedDeclarationArtifact<D, I>,
    claim: WorthQueryGroupedSharedPostureClaim,
) -> bool {
    match claim {
        WorthQueryGroupedSharedPostureClaim::SharedSelectionFocus => {
            let required_field = grouped_support_field_key("selection", "active_face");
            declaration
                .aspect_participation()
                .present_all()
                .iter()
                .any(|value| value == &required_field)
        }
        WorthQueryGroupedSharedPostureClaim::SharedMaterialPreview => {
            let required_field = grouped_support_field_key("selection", "material_preview");
            declaration
                .aspect_participation()
                .present_all()
                .iter()
                .any(|value| value == &required_field)
                && declaration.grouping_intent() == WorthQueryGroupedIntent::Authoritative
        }
        WorthQueryGroupedSharedPostureClaim::SharedContinuity => {
            declaration.continuity_assumption()
                == WorthQueryGroupedContinuityAssumption::PreserveNeighborhood
        }
    }
}

fn grouped_support_field_key(aspect: &str, field: &str) -> AspectFieldKey {
    let aspect_key = AspectKey::new(aspect).expect("grouped support aspect key must admit");
    let field_key = FieldKey::new(field).expect("grouped support field key must admit");
    AspectFieldKey::from_native_keys(&aspect_key, &field_key)
}
