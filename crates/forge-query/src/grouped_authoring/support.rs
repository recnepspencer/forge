use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::authoring::AspectFieldKey;
use forge_foundational::facade::{AspectKey, FieldKey};

use super::artifact::ForgeQueryGroupedDeclarationArtifact;
use super::posture::{
    ForgeQueryGroupedContinuityAssumption, ForgeQueryGroupedIntent,
    ForgeQueryGroupedSharedPostureClaim,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedSupportFeature {
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

impl ForgeQueryGroupedSupportFeature {
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
pub enum ForgeQueryGroupedSupportStatus {
    Supported,
    Unsupported,
}

impl ForgeQueryGroupedSupportStatus {
    pub fn is_supported(self) -> bool {
        self == Self::Supported
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedSupportReport {
    statuses: Vec<(
        ForgeQueryGroupedSupportFeature,
        ForgeQueryGroupedSupportStatus,
    )>,
    unsupported_claims: Vec<ForgeQueryGroupedSharedPostureClaim>,
}

impl ForgeQueryGroupedSupportReport {
    fn new(
        statuses: Vec<(
            ForgeQueryGroupedSupportFeature,
            ForgeQueryGroupedSupportStatus,
        )>,
        unsupported_claims: Vec<ForgeQueryGroupedSharedPostureClaim>,
    ) -> Self {
        Self {
            statuses,
            unsupported_claims,
        }
    }

    pub fn statuses(
        &self,
    ) -> &[(
        ForgeQueryGroupedSupportFeature,
        ForgeQueryGroupedSupportStatus,
    )] {
        &self.statuses
    }

    pub fn status_for(
        &self,
        feature: ForgeQueryGroupedSupportFeature,
    ) -> ForgeQueryGroupedSupportStatus {
        self.statuses
            .iter()
            .find(|(candidate, _)| *candidate == feature)
            .map(|(_, status)| *status)
            .unwrap_or(ForgeQueryGroupedSupportStatus::Unsupported)
    }

    pub fn unsupported_claims(&self) -> &[ForgeQueryGroupedSharedPostureClaim] {
        &self.unsupported_claims
    }
}

pub(crate) fn forge_query_grouped_support_report<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedSupportReport {
    let unsupported_claims = declaration
        .shared_posture_claims()
        .iter()
        .copied()
        .filter(|claim| !claim_supported(declaration, *claim))
        .collect::<Vec<_>>();
    let shared_status = if unsupported_claims.is_empty() {
        ForgeQueryGroupedSupportStatus::Supported
    } else {
        ForgeQueryGroupedSupportStatus::Unsupported
    };
    ForgeQueryGroupedSupportReport::new(
        vec![
            (
                ForgeQueryGroupedSupportFeature::GroupedDeclaration,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::GroupedRoute,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::GroupedReceipt,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::GroupedEnvelope,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::GroupedContributionComposition,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::Atomicity,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::GroupingIntent,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::ContinuityAssumption,
                ForgeQueryGroupedSupportStatus::Supported,
            ),
            (
                ForgeQueryGroupedSupportFeature::SharedPostureClaims,
                shared_status,
            ),
        ],
        unsupported_claims,
    )
}

fn claim_supported<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    declaration: &ForgeQueryGroupedDeclarationArtifact<D, I>,
    claim: ForgeQueryGroupedSharedPostureClaim,
) -> bool {
    match claim {
        ForgeQueryGroupedSharedPostureClaim::SharedSelectionFocus => {
            let required_field = grouped_support_field_key("selection", "active_face");
            declaration
                .aspect_participation()
                .present_all()
                .iter()
                .any(|value| value == &required_field)
        }
        ForgeQueryGroupedSharedPostureClaim::SharedMaterialPreview => {
            let required_field = grouped_support_field_key("selection", "material_preview");
            declaration
                .aspect_participation()
                .present_all()
                .iter()
                .any(|value| value == &required_field)
                && declaration.grouping_intent() == ForgeQueryGroupedIntent::Authoritative
        }
        ForgeQueryGroupedSharedPostureClaim::SharedContinuity => {
            declaration.continuity_assumption()
                == ForgeQueryGroupedContinuityAssumption::PreserveNeighborhood
        }
    }
}

fn grouped_support_field_key(aspect: &str, field: &str) -> AspectFieldKey {
    let aspect_key = AspectKey::new(aspect).expect("grouped support aspect key must admit");
    let field_key = FieldKey::new(field).expect("grouped support field key must admit");
    AspectFieldKey::from_native_keys(&aspect_key, &field_key)
}
