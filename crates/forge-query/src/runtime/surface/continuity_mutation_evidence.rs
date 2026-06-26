use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationEvidenceDigest,
    ForgeQueryMutationTargetCollectionIdentity,
};
use forge_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeContinuityMutationFamily, BridgeContinuityOutcomeClass,
    RelationalBridgeRecordIdentityParts,
};

#[cfg(test)]
#[path = "continuity_mutation_evidence/test_support.rs"]
mod test_support;

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
    prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    successor_authoritative_identities: Vec<ForgeQueryMutationAuthorityIdentity>,
    basis_binding_digest: Option<ForgeQueryMutationEvidenceDigest>,
    resolved_target_entity_identity: Option<ForgeQueryEntityIdentity>,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    lineage_digest: ForgeQueryMutationEvidenceDigest,
    continuity_resolution_digest: ForgeQueryMutationEvidenceDigest,
}

impl ForgeQueryContinuityMutationEvidence {
    #[cfg(test)]
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeContinuityMutationBundle) -> Self {
        Self::from_bridge_with_query_context(bundle, None, None, None)
    }

    pub(in crate::runtime) fn from_bridge_with_query_context(
        bundle: &BridgeContinuityMutationBundle,
        basis_binding_identity: Option<&ForgeQueryEvidenceIdentity>,
        resolved_target_entity_identity: Option<&ForgeQueryEntityIdentity>,
        target_collection: Option<&ForgeQueryMutationTargetCollectionIdentity>,
    ) -> Self {
        let family = match bundle.family() {
            BridgeContinuityMutationFamily::RebindExistingTarget => {
                crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget
            }
            BridgeContinuityMutationFamily::SplitExistingTarget => {
                crate::runtime::ForgeQueryContinuityMutationFamily::SplitExistingTarget
            }
        };
        let outcome_class = map_outcome_class(bundle.outcome_class());
        let prior_authoritative_identity =
            ForgeQueryMutationAuthorityIdentity::from_bridge_continuity_authority(
                "continuity-prior",
                bundle.prior_authoritative_identity_handle(),
            );
        let successor_authoritative_identities = bundle
            .successor_authoritative_identities()
            .iter()
            .map(|value| {
                ForgeQueryMutationAuthorityIdentity::from_bridge_continuity_authority(
                    "continuity-successor",
                    value,
                )
            })
            .collect::<Vec<_>>();
        let basis_binding_digest = basis_binding_identity.map(|identity| {
            ForgeQueryMutationEvidenceDigest::source_identity("continuity-basis-binding", identity)
        });
        let resolved_target_entity_identity = bundle
            .resolved_target_entity_identity()
            .and_then(RelationalBridgeRecordIdentityParts::from_bridge_entity_identity)
            .map(ForgeQueryEntityIdentity::from_relational_record)
            .or_else(|| resolved_target_entity_identity.cloned());
        let target_collection = bundle
            .target_collection()
            .map(|collection| {
                ForgeQueryMutationTargetCollectionIdentity::new("continuity-target", collection)
            })
            .or_else(|| target_collection.cloned());
        let lineage_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "continuity-lineage")
                .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("outcome"),
                    outcome_class.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("prior"),
                    prior_authoritative_identity.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(ForgeQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(ForgeQueryMutationEvidenceDigest::evidence_identity),
                )
                .seal();
        let lineage_digest =
            ForgeQueryMutationEvidenceDigest::aggregate("continuity-lineage", lineage_identity);
        let resolved_target_identity = resolved_target_entity_identity
            .as_ref()
            .map(ForgeQueryEntityIdentity::evidence_identity);
        let resolution_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "continuity-resolution")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("lineage"),
                    lineage_digest.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(ForgeQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(ForgeQueryMutationEvidenceDigest::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved"),
                    resolved_target_identity.as_ref(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("collection"),
                    target_collection
                        .as_ref()
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .seal();
        let continuity_resolution_digest = ForgeQueryMutationEvidenceDigest::aggregate(
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
        intent: &crate::runtime::ForgeQueryContinuityMutationIntent,
        basis_binding_identity: Option<&ForgeQueryEvidenceIdentity>,
        resolved_target_entity_identity: Option<&ForgeQueryEntityIdentity>,
        target_collection: Option<&ForgeQueryMutationTargetCollectionIdentity>,
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
        let prior_authoritative_identity = intent.prior_authoritative_identity().clone();
        let successor_authoritative_identities = intent
            .successor_authoritative_identities()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        let basis_binding_digest = basis_binding_identity.map(|identity| {
            ForgeQueryMutationEvidenceDigest::source_identity("continuity-basis-binding", identity)
        });
        let target_collection = target_collection.cloned();
        let lineage_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "continuity-lineage")
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    intent.family().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("outcome"),
                    intent.outcome_class().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("prior"),
                    prior_authoritative_identity.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(ForgeQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(ForgeQueryMutationEvidenceDigest::evidence_identity),
                )
                .seal();
        let lineage_digest =
            ForgeQueryMutationEvidenceDigest::aggregate("continuity-lineage", lineage_identity);
        let resolved_target_identity =
            resolved_target_entity_identity.map(ForgeQueryEntityIdentity::evidence_identity);
        let resolution_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "continuity-resolution")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("lineage"),
                    lineage_digest.evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("successor"),
                    successor_authoritative_identities
                        .iter()
                        .map(ForgeQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_binding"),
                    basis_binding_digest
                        .as_ref()
                        .map(ForgeQueryMutationEvidenceDigest::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved"),
                    resolved_target_identity.as_ref(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("collection"),
                    target_collection
                        .as_ref()
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .seal();
        let continuity_resolution_digest = ForgeQueryMutationEvidenceDigest::aggregate(
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

    pub fn prior_authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identity(&self) -> Option<&ForgeQueryMutationAuthorityIdentity> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[ForgeQueryMutationAuthorityIdentity] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&ForgeQueryMutationEvidenceDigest> {
        self.basis_binding_digest.as_ref()
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&ForgeQueryEntityIdentity> {
        self.resolved_target_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }

    pub fn lineage_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.lineage_digest
    }

    pub fn continuity_resolution_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.continuity_resolution_digest
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
