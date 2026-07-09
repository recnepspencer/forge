use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::runtime::{
    WorthQueryMutationAuthorityIdentity, WorthQueryMutationEvidenceDigest,
    WorthQueryMutationTargetCollectionIdentity,
};
use worth_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeContinuityMutationFamily, BridgeContinuityOutcomeClass,
    RelationalBridgeRecordIdentityParts,
};

#[cfg(test)]
#[path = "continuity_mutation_evidence/test_support.rs"]
mod test_support;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryContinuityClass {
    SingleSuccessor,
    SplitSuccessors,
    TruthLoweredCanonicalMergeSuccessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryContinuityRejectionClass {
    NoAuthoritativeSuccessor,
    AmbiguousSuccessor,
    UnsupportedContinuityClass,
    HistoricalResolutionFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryContinuityOutcomeClass {
    ContinuesAsSingleSuccessor,
    ContinuesAsSplitSuccessors,
    ContinuesViaTruthLoweredCanonicalMergeSuccessor,
    RejectedNoAuthoritativeSuccessor,
    RejectedAmbiguousSuccessor,
    RejectedUnsupportedContinuityClass,
    RejectedHistoricalResolutionFailure,
}

impl WorthQueryContinuityOutcomeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContinuesAsSingleSuccessor => "continues_as_single_successor",
            Self::ContinuesAsSplitSuccessors => "continues_as_split_successors",
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                "continues_via_truth_lowered_canonical_merge_successor"
            }
            Self::RejectedNoAuthoritativeSuccessor => "rejected_no_authoritative_successor",
            Self::RejectedAmbiguousSuccessor => "rejected_ambiguous_successor",
            Self::RejectedUnsupportedContinuityClass => "rejected_unsupported_continuity_class",
            Self::RejectedHistoricalResolutionFailure => "rejected_historical_resolution_failure",
        }
    }

    pub fn continuity_class(self) -> Option<WorthQueryContinuityClass> {
        match self {
            Self::ContinuesAsSingleSuccessor => Some(WorthQueryContinuityClass::SingleSuccessor),
            Self::ContinuesAsSplitSuccessors => Some(WorthQueryContinuityClass::SplitSuccessors),
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                Some(WorthQueryContinuityClass::TruthLoweredCanonicalMergeSuccessor)
            }
            Self::RejectedNoAuthoritativeSuccessor
            | Self::RejectedAmbiguousSuccessor
            | Self::RejectedUnsupportedContinuityClass
            | Self::RejectedHistoricalResolutionFailure => None,
        }
    }

    pub fn rejection_class(self) -> Option<WorthQueryContinuityRejectionClass> {
        match self {
            Self::ContinuesAsSingleSuccessor
            | Self::ContinuesAsSplitSuccessors
            | Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => None,
            Self::RejectedNoAuthoritativeSuccessor => {
                Some(WorthQueryContinuityRejectionClass::NoAuthoritativeSuccessor)
            }
            Self::RejectedAmbiguousSuccessor => {
                Some(WorthQueryContinuityRejectionClass::AmbiguousSuccessor)
            }
            Self::RejectedUnsupportedContinuityClass => {
                Some(WorthQueryContinuityRejectionClass::UnsupportedContinuityClass)
            }
            Self::RejectedHistoricalResolutionFailure => {
                Some(WorthQueryContinuityRejectionClass::HistoricalResolutionFailure)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContinuityMutationEvidence {
    family: crate::runtime::WorthQueryContinuityMutationFamily,
    outcome_class: WorthQueryContinuityOutcomeClass,
    prior_authoritative_identity: WorthQueryMutationAuthorityIdentity,
    successor_authoritative_identities: Vec<WorthQueryMutationAuthorityIdentity>,
    basis_binding_digest: Option<WorthQueryMutationEvidenceDigest>,
    resolved_target_entity_identity: Option<WorthQueryEntityIdentity>,
    target_collection: Option<WorthQueryMutationTargetCollectionIdentity>,
    lineage_digest: WorthQueryMutationEvidenceDigest,
    continuity_resolution_digest: WorthQueryMutationEvidenceDigest,
}

impl WorthQueryContinuityMutationEvidence {
    #[cfg(test)]
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeContinuityMutationBundle) -> Self {
        Self::from_bridge_with_query_context(bundle, None, None, None)
    }

    pub(in crate::runtime) fn from_bridge_with_query_context(
        bundle: &BridgeContinuityMutationBundle,
        basis_binding_identity: Option<&WorthQueryEvidenceIdentity>,
        resolved_target_entity_identity: Option<&WorthQueryEntityIdentity>,
        target_collection: Option<&WorthQueryMutationTargetCollectionIdentity>,
    ) -> Self {
        let family = match bundle.family() {
            BridgeContinuityMutationFamily::RebindExistingTarget => {
                crate::runtime::WorthQueryContinuityMutationFamily::RebindExistingTarget
            }
            BridgeContinuityMutationFamily::SplitExistingTarget => {
                crate::runtime::WorthQueryContinuityMutationFamily::SplitExistingTarget
            }
        };
        let outcome_class = map_outcome_class(bundle.outcome_class());
        let prior_authoritative_identity =
            WorthQueryMutationAuthorityIdentity::from_bridge_continuity_authority(
                "continuity-prior",
                bundle.prior_authoritative_identity_handle(),
            );
        let successor_authoritative_identities = bundle
            .successor_authoritative_identities()
            .iter()
            .map(|value| {
                WorthQueryMutationAuthorityIdentity::from_bridge_continuity_authority(
                    "continuity-successor",
                    value,
                )
            })
            .collect::<Vec<_>>();
        let basis_binding_digest = basis_binding_identity.map(|identity| {
            WorthQueryMutationEvidenceDigest::source_identity("continuity-basis-binding", identity)
        });
        let resolved_target_entity_identity = bundle
            .resolved_target_entity_identity()
            .and_then(RelationalBridgeRecordIdentityParts::from_bridge_entity_identity)
            .map(WorthQueryEntityIdentity::from_relational_record)
            .or_else(|| resolved_target_entity_identity.cloned());
        let target_collection = bundle
            .target_collection()
            .map(|collection| {
                WorthQueryMutationTargetCollectionIdentity::new("continuity-target", collection)
            })
            .or_else(|| target_collection.cloned());
        let lineage_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(WorthQueryEvidenceTag::new("role"), "continuity-lineage")
                .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
                .field_shape(
                    WorthQueryEvidenceTag::new("outcome"),
                    outcome_class.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("prior"),
                    prior_authoritative_identity.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(WorthQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(WorthQueryMutationEvidenceDigest::evidence_identity),
                )
                .seal();
        let lineage_digest =
            WorthQueryMutationEvidenceDigest::aggregate("continuity-lineage", lineage_identity);
        let resolved_target_identity = resolved_target_entity_identity
            .as_ref()
            .map(WorthQueryEntityIdentity::evidence_identity);
        let resolution_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(WorthQueryEvidenceTag::new("role"), "continuity-resolution")
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("lineage"),
                    lineage_digest.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(WorthQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(WorthQueryMutationEvidenceDigest::evidence_identity),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("resolved"),
                    resolved_target_identity.as_ref(),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("collection"),
                    target_collection
                        .as_ref()
                        .map(WorthQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .seal();
        let continuity_resolution_digest = WorthQueryMutationEvidenceDigest::aggregate(
            "continuity-resolution",
            resolution_identity,
        );
        Self {
            family,
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identities,
            basis_binding_digest,
            resolved_target_entity_identity,
            target_collection,
            lineage_digest,
            continuity_resolution_digest,
        }
    }

    pub(crate) fn from_intent(
        intent: &crate::runtime::WorthQueryContinuityMutationIntent,
        basis_binding_identity: Option<&WorthQueryEvidenceIdentity>,
        resolved_target_entity_identity: Option<&WorthQueryEntityIdentity>,
        target_collection: Option<&WorthQueryMutationTargetCollectionIdentity>,
    ) -> Self {
        let outcome_class = match intent.outcome_class() {
            crate::runtime::WorthQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor => {
                WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            }
            crate::runtime::WorthQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors => {
                WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
            }
            crate::runtime::WorthQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
            }
        };
        let prior_authoritative_identity = intent.prior_authoritative_identity().clone();
        let successor_authoritative_identities = intent
            .successor_authoritative_identities()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        let basis_binding_digest = basis_binding_identity.map(|identity| {
            WorthQueryMutationEvidenceDigest::source_identity("continuity-basis-binding", identity)
        });
        let target_collection = target_collection.cloned();
        let lineage_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(WorthQueryEvidenceTag::new("role"), "continuity-lineage")
                .field_shape(
                    WorthQueryEvidenceTag::new("family"),
                    intent.family().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("outcome"),
                    intent.outcome_class().as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("prior"),
                    prior_authoritative_identity.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(WorthQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(WorthQueryMutationEvidenceDigest::evidence_identity),
                )
                .seal();
        let lineage_digest =
            WorthQueryMutationEvidenceDigest::aggregate("continuity-lineage", lineage_identity);
        let resolved_target_identity =
            resolved_target_entity_identity.map(WorthQueryEntityIdentity::evidence_identity);
        let resolution_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(WorthQueryEvidenceTag::new("role"), "continuity-resolution")
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("lineage"),
                    lineage_digest.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(WorthQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(WorthQueryMutationEvidenceDigest::evidence_identity),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("resolved"),
                    resolved_target_identity.as_ref(),
                )
                .optional_evidence_identity(
                    WorthQueryEvidenceTag::new("collection"),
                    target_collection
                        .as_ref()
                        .map(WorthQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .seal();
        let continuity_resolution_digest = WorthQueryMutationEvidenceDigest::aggregate(
            "continuity-resolution",
            resolution_identity,
        );
        Self {
            family: intent.family(),
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identities,
            basis_binding_digest,
            resolved_target_entity_identity: resolved_target_entity_identity.cloned(),
            target_collection,
            lineage_digest,
            continuity_resolution_digest,
        }
    }

    pub fn family(&self) -> crate::runtime::WorthQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> WorthQueryContinuityOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> Option<WorthQueryContinuityClass> {
        self.outcome_class.continuity_class()
    }

    pub fn rejection_class(&self) -> Option<WorthQueryContinuityRejectionClass> {
        self.outcome_class.rejection_class()
    }

    pub fn prior_authoritative_identity(&self) -> &WorthQueryMutationAuthorityIdentity {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identity(&self) -> Option<&WorthQueryMutationAuthorityIdentity> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[WorthQueryMutationAuthorityIdentity] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&WorthQueryMutationEvidenceDigest> {
        self.basis_binding_digest.as_ref()
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&WorthQueryEntityIdentity> {
        self.resolved_target_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&WorthQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }

    pub fn lineage_digest(&self) -> &WorthQueryMutationEvidenceDigest {
        &self.lineage_digest
    }

    pub fn continuity_resolution_digest(&self) -> &WorthQueryMutationEvidenceDigest {
        &self.continuity_resolution_digest
    }
}

fn map_outcome_class(outcome: BridgeContinuityOutcomeClass) -> WorthQueryContinuityOutcomeClass {
    match outcome {
        BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
            WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
        }
        BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
            WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
        }
        BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor => {
            WorthQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
            WorthQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass => {
            WorthQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass
        }
        BridgeContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
            WorthQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure
        }
    }
}
