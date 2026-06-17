use crate::application::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionEvidence,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::intent_result::{
    ForgeQueryContributionComposedIntentClassification, ForgeQueryContributionComposedIntentResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionComposedClassification {
    FullyAdmitted,
    PartiallyAdmitted,
    NoContributionAdmitted,
    MaterializationFailedAfterAdmission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionComposedStop {
    Deferred,
    DeclarationDenied,
    ContributionDenied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedComposition {
    classification: ForgeQueryContributionComposedClassification,
    admitted_category_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
    rejected_category_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
    admitted_evidence: Vec<ForgeQueryDeclarationEntryContributionEvidence>,
    composition_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryContributionComposedComposition {
    pub fn from_intent_results(
        classification: ForgeQueryContributionComposedClassification,
        intent_results: &[ForgeQueryContributionComposedIntentResult],
    ) -> Self {
        let admitted_evidence = intent_results
            .iter()
            .filter_map(|value| value.contribution().map(|entry| entry.evidence().clone()))
            .collect::<Vec<_>>();
        let admitted_category_families = admitted_evidence
            .iter()
            .map(ForgeQueryDeclarationEntryContributionEvidence::category_family)
            .collect::<Vec<_>>();
        let rejected_category_families = intent_results
            .iter()
            .filter(|value| !value.is_admitted())
            .map(ForgeQueryContributionComposedIntentResult::category_family)
            .collect::<Vec<_>>();
        let composition_identity = compose_composition_identity(classification, intent_results);
        Self {
            classification,
            admitted_category_families,
            rejected_category_families,
            admitted_evidence,
            composition_identity,
        }
    }

    pub fn classification(&self) -> ForgeQueryContributionComposedClassification {
        self.classification
    }

    pub fn admitted_category_families(
        &self,
    ) -> &[ForgeQueryDeclarationEntryContributionCategoryFamily] {
        &self.admitted_category_families
    }

    pub fn rejected_category_families(
        &self,
    ) -> &[ForgeQueryDeclarationEntryContributionCategoryFamily] {
        &self.rejected_category_families
    }

    pub fn admitted_evidence(&self) -> &[ForgeQueryDeclarationEntryContributionEvidence] {
        &self.admitted_evidence
    }

    pub fn composition_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.composition_identity
    }

    pub fn composition_for_reporting(&self) -> &str {
        self.composition_identity.as_str()
    }
}

fn compose_intent_result_identity(
    value: &ForgeQueryContributionComposedIntentResult,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_contribution_composed_intent_result_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("category_family"),
            value.category_family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("classification"),
            format!("{:?}", value.classification()),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("request"),
            value.request_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("binding"),
            value.binding_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("declaration_contract"),
            format!("{:?}", value.aspect_record().declaration_contract()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("declaration_coverage"),
            format!("{:?}", value.aspect_record().declaration_coverage()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("evaluation"),
            value.evaluation().stage_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("admission"),
            value.admission().stage_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("materialization"),
            value
                .materialization()
                .stage_identity()
                .or_else(|| value.admission().stage_identity())
                .or_else(|| value.evaluation().stage_identity()),
        )
        .seal()
}

fn compose_composition_identity(
    classification: ForgeQueryContributionComposedClassification,
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> ForgeQueryEvidenceIdentity {
    let mut intent_identities = intent_results
        .iter()
        .map(compose_intent_result_identity)
        .collect::<Vec<_>>();
    intent_identities.sort_by_key(|identity| identity.as_str().to_owned());
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_contribution_composed_composition_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("classification"),
            format!("{classification:?}"),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("intent_results"),
            intent_identities.iter(),
        )
        .seal()
}

pub fn classify_intent_results(
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> ForgeQueryContributionComposedClassification {
    let admitted_count = intent_results
        .iter()
        .filter(|value| value.is_admitted())
        .count();
    let has_materialization_failure = intent_results.iter().any(|value| {
        value.classification()
            == ForgeQueryContributionComposedIntentClassification::MaterializationFailedAfterAdmission
    });
    if admitted_count == intent_results.len() {
        ForgeQueryContributionComposedClassification::FullyAdmitted
    } else if has_materialization_failure {
        ForgeQueryContributionComposedClassification::MaterializationFailedAfterAdmission
    } else if admitted_count == 0 {
        ForgeQueryContributionComposedClassification::NoContributionAdmitted
    } else {
        ForgeQueryContributionComposedClassification::PartiallyAdmitted
    }
}

pub fn strongest_stop(
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> ForgeQueryContributionComposedStop {
    if intent_results.iter().any(|value| {
        value.classification() == ForgeQueryContributionComposedIntentClassification::Denied
    }) {
        ForgeQueryContributionComposedStop::ContributionDenied
    } else if intent_results.iter().any(|value| {
        value.classification() == ForgeQueryContributionComposedIntentClassification::Stale
    }) {
        ForgeQueryContributionComposedStop::Stale
    } else if intent_results.iter().any(|value| {
        value.classification() == ForgeQueryContributionComposedIntentClassification::RebindRequired
    }) {
        ForgeQueryContributionComposedStop::RebindRequired
    } else if intent_results.iter().any(|value| {
        value.classification() == ForgeQueryContributionComposedIntentClassification::Unsupported
    }) {
        ForgeQueryContributionComposedStop::Unsupported
    } else {
        ForgeQueryContributionComposedStop::Failed
    }
}
