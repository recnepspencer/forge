use crate::application::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionEvidence,
};
use crate::identity::hash_parts;

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
    composition_digest: String,
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
        let mut digest_parts = intent_results
            .iter()
            .map(|value| {
                format!(
                    "{}:{:?}:{}:{}:{:?}:{:?}",
                    value.request_digest(),
                    value.classification(),
                    value.target_binding_digest(),
                    value
                        .materialization()
                        .digest()
                        .or_else(|| value.admission().digest())
                        .or_else(|| value.evaluation().digest())
                        .unwrap_or("none"),
                    value.aspect_record().declaration_contract(),
                    value.aspect_record().declaration_coverage(),
                )
            })
            .collect::<Vec<_>>();
        digest_parts.sort();
        digest_parts.insert(0, format!("classification:{:?}", classification));
        let composition_digest = hash_parts(&digest_parts);
        Self {
            classification,
            admitted_category_families,
            rejected_category_families,
            admitted_evidence,
            composition_digest,
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

    pub fn composition_digest(&self) -> &str {
        &self.composition_digest
    }
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
