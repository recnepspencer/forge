use forge_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeContinuityMutationFamily, BridgeContinuityOutcomeClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryContinuityClass {
    SingleSuccessor,
    SplitSuccessors,
    TruthLoweredCanonicalMergeSuccessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryContinuityRejectionClass {
    NoAuthoritativeSuccessor,
    AmbiguousSuccessor,
    UnsupportedContinuityClass,
    HistoricalResolutionFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryContinuityOutcomeClass {
    ContinuesAsSingleSuccessor,
    ContinuesAsSplitSuccessors,
    ContinuesViaTruthLoweredCanonicalMergeSuccessor,
    RejectedNoAuthoritativeSuccessor,
    RejectedAmbiguousSuccessor,
    RejectedUnsupportedContinuityClass,
    RejectedHistoricalResolutionFailure,
}

impl ForgeQueryContinuityOutcomeClass {
    pub fn continuity_class(self) -> Option<ForgeQueryContinuityClass> {
        match self {
            Self::ContinuesAsSingleSuccessor => Some(ForgeQueryContinuityClass::SingleSuccessor),
            Self::ContinuesAsSplitSuccessors => Some(ForgeQueryContinuityClass::SplitSuccessors),
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                Some(ForgeQueryContinuityClass::TruthLoweredCanonicalMergeSuccessor)
            }
            Self::RejectedNoAuthoritativeSuccessor
            | Self::RejectedAmbiguousSuccessor
            | Self::RejectedUnsupportedContinuityClass
            | Self::RejectedHistoricalResolutionFailure => None,
        }
    }

    pub fn rejection_class(self) -> Option<ForgeQueryContinuityRejectionClass> {
        match self {
            Self::ContinuesAsSingleSuccessor
            | Self::ContinuesAsSplitSuccessors
            | Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => None,
            Self::RejectedNoAuthoritativeSuccessor => {
                Some(ForgeQueryContinuityRejectionClass::NoAuthoritativeSuccessor)
            }
            Self::RejectedAmbiguousSuccessor => {
                Some(ForgeQueryContinuityRejectionClass::AmbiguousSuccessor)
            }
            Self::RejectedUnsupportedContinuityClass => {
                Some(ForgeQueryContinuityRejectionClass::UnsupportedContinuityClass)
            }
            Self::RejectedHistoricalResolutionFailure => {
                Some(ForgeQueryContinuityRejectionClass::HistoricalResolutionFailure)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuityMutationEvidence {
    family: crate::runtime::ForgeQueryContinuityMutationFamily,
    outcome_class: ForgeQueryContinuityOutcomeClass,
    prior_authoritative_identity: String,
    successor_authoritative_identities: Vec<String>,
    basis_binding_digest: Option<String>,
    resolved_target_entity_identity: Option<String>,
    target_collection: Option<String>,
    lineage_digest: String,
    continuity_resolution_digest: String,
}

impl ForgeQueryContinuityMutationEvidence {
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeContinuityMutationBundle) -> Self {
        Self {
            family: match bundle.family() {
                BridgeContinuityMutationFamily::RebindExistingTarget => {
                    crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget
                }
                BridgeContinuityMutationFamily::SplitExistingTarget => {
                    crate::runtime::ForgeQueryContinuityMutationFamily::SplitExistingTarget
                }
            },
            outcome_class: map_outcome_class(bundle.outcome_class()),
            prior_authoritative_identity: bundle.prior_authoritative_identity().to_string(),
            successor_authoritative_identities: bundle
                .successor_authoritative_identities()
                .iter()
                .map(|value| value.as_ref().to_string())
                .collect(),
            basis_binding_digest: bundle.basis_binding_digest().map(str::to_string),
            resolved_target_entity_identity: bundle
                .resolved_target_entity_identity()
                .map(str::to_string),
            target_collection: bundle.target_collection().map(str::to_string),
            lineage_digest: bundle.lineage_digest().to_string(),
            continuity_resolution_digest: bundle.continuity_resolution_digest().to_string(),
        }
    }

    pub(in crate::runtime) fn from_intent(
        intent: &crate::runtime::ForgeQueryContinuityMutationIntent,
        basis_binding_digest: Option<&str>,
        resolved_target_entity_identity: Option<&str>,
        target_collection: Option<&str>,
    ) -> Self {
        let outcome_class = match intent.outcome_class() {
            crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor => {
                ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            }
            crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors => {
                ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
            }
            crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                ForgeQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
            }
        };
        let lineage_digest = crate::identity::hash_parts(&[
            "forge-query-continuity-lineage-v1".to_string(),
            format!("family:{}", intent.family().as_str()),
            format!("outcome:{}", intent.outcome_class().as_str()),
            format!("prior:{}", intent.prior_authoritative_identity()),
            format!(
                "successors:{}",
                intent.successor_authoritative_identities().join("|")
            ),
            format!("basis-binding:{}", basis_binding_digest.unwrap_or("none")),
        ]);
        let continuity_resolution_digest = crate::identity::hash_parts(&[
            "forge-query-continuity-resolution-v1".to_string(),
            format!("lineage:{lineage_digest}"),
            format!(
                "successors:{}",
                intent.successor_authoritative_identities().join("|")
            ),
            format!("basis-binding:{}", basis_binding_digest.unwrap_or("none")),
            format!(
                "resolved:{}",
                resolved_target_entity_identity.unwrap_or("none")
            ),
            format!("collection:{}", target_collection.unwrap_or("none")),
        ]);
        Self {
            family: intent.family(),
            outcome_class,
            prior_authoritative_identity: intent.prior_authoritative_identity().to_string(),
            successor_authoritative_identities: intent
                .successor_authoritative_identities()
                .to_vec(),
            basis_binding_digest: basis_binding_digest.map(str::to_string),
            resolved_target_entity_identity: resolved_target_entity_identity.map(str::to_string),
            target_collection: target_collection.map(str::to_string),
            lineage_digest,
            continuity_resolution_digest,
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> ForgeQueryContinuityOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> Option<ForgeQueryContinuityClass> {
        self.outcome_class.continuity_class()
    }

    pub fn rejection_class(&self) -> Option<ForgeQueryContinuityRejectionClass> {
        self.outcome_class.rejection_class()
    }

    pub fn prior_authoritative_identity(&self) -> &str {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identity(&self) -> Option<&str> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only.as_str()),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[String] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&str> {
        self.basis_binding_digest.as_deref()
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&str> {
        self.resolved_target_entity_identity.as_deref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn lineage_digest(&self) -> &str {
        &self.lineage_digest
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        &self.continuity_resolution_digest
    }

    #[cfg(test)]
    pub(crate) fn with_test_family(
        mut self,
        family: crate::runtime::ForgeQueryContinuityMutationFamily,
    ) -> Self {
        self.family = family;
        self
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        family: crate::runtime::ForgeQueryContinuityMutationFamily,
        outcome_class: ForgeQueryContinuityOutcomeClass,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: Vec<String>,
        resolved_target_entity_identity: Option<&str>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family,
            outcome_class,
            prior_authoritative_identity: prior_authoritative_identity.into(),
            successor_authoritative_identities,
            basis_binding_digest: Some("basis-binding:test".to_string()),
            resolved_target_entity_identity: resolved_target_entity_identity.map(str::to_string),
            target_collection: target_collection.map(str::to_string),
            lineage_digest: "lineage:test".to_string(),
            continuity_resolution_digest: "continuity-resolution:test".to_string(),
        }
    }
}

fn map_outcome_class(outcome: BridgeContinuityOutcomeClass) -> ForgeQueryContinuityOutcomeClass {
    match outcome {
        BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
            ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
        }
        BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
            ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
        }
        BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            ForgeQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor => {
            ForgeQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
            ForgeQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass => {
            ForgeQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass
        }
        BridgeContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
            ForgeQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure
        }
    }
}
