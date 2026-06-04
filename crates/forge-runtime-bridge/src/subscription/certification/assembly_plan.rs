use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationSemanticSourceDigestSet,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactIndex,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BridgeSubscriptionCertificationFieldExpectation {
    CompleteReferenceBundle,
    RetainedArtifactCompletenessRequirement,
}

impl BridgeSubscriptionCertificationFieldExpectation {
    const fn expected_field_count(self) -> usize {
        match self {
            Self::CompleteReferenceBundle => 8,
            Self::RetainedArtifactCompletenessRequirement => 9,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteReferenceBundle => "complete_reference_bundle",
            Self::RetainedArtifactCompletenessRequirement => {
                "retained_artifact_completeness_requirement"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BridgeSubscriptionCertificationBundleSchemaIdentity {
    Current,
    DivergentSchemaParityIdentity,
}

impl BridgeSubscriptionCertificationBundleSchemaIdentity {
    const fn schema_version(self) -> &'static str {
        match self {
            Self::Current => super::bundle::BRIDGE_SUBSCRIPTION_CERTIFICATION_BUNDLE_SCHEMA_V1,
            Self::DivergentSchemaParityIdentity => "bridge-subscription-certification-bundle-v999",
        }
    }

    const fn digest_algorithm(self) -> &'static str {
        match self {
            Self::Current => super::bundle::BRIDGE_SUBSCRIPTION_CERTIFICATION_DIGEST_ALGORITHM_V1,
            Self::DivergentSchemaParityIdentity => "sha512",
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DivergentSchemaParityIdentity => "divergent_schema_parity_identity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationAssemblyPlan {
    manifest_digest: Arc<str>,
    source_artifact_index_digest: Arc<str>,
    bundle_schema_version: Arc<str>,
    bundle_digest_algorithm: Arc<str>,
    semantic_source_digests: BridgeSubscriptionCertificationSemanticSourceDigestSet,
    selected_record_count: usize,
    expected_field_count: usize,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationAssemblyPlan {
    pub(crate) fn plan(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
    ) -> Self {
        Self::plan_with_field_expectation(
            manifest,
            source_artifact_index,
            BridgeSubscriptionCertificationFieldExpectation::CompleteReferenceBundle,
        )
    }

    pub(crate) fn plan_with_field_expectation(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
        field_expectation: BridgeSubscriptionCertificationFieldExpectation,
    ) -> Self {
        Self::plan_with_bundle_identity(
            manifest,
            source_artifact_index,
            field_expectation,
            BridgeSubscriptionCertificationBundleSchemaIdentity::Current,
        )
    }

    pub(crate) fn plan_with_bundle_identity(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
        field_expectation: BridgeSubscriptionCertificationFieldExpectation,
        bundle_schema_identity: BridgeSubscriptionCertificationBundleSchemaIdentity,
    ) -> Self {
        let selected_record_count = source_artifact_index.records().len();
        let expected_field_count = field_expectation.expected_field_count();
        let semantic_source_digests =
            BridgeSubscriptionCertificationSemanticSourceDigestSet::from_source_artifact_index(
                source_artifact_index,
            );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-assembly-plan|manifest={}|source-index={}|schema-identity={}|schema={}|algorithm={}|semantic-sources={}|records={}|field-expectation={}|fields={}",
            manifest.digest(),
            source_artifact_index.digest(),
            bundle_schema_identity.as_str(),
            bundle_schema_identity.schema_version(),
            bundle_schema_identity.digest_algorithm(),
            semantic_source_digests.digest(),
            selected_record_count,
            field_expectation.as_str(),
            expected_field_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            manifest_digest: Arc::from(manifest.digest()),
            source_artifact_index_digest: Arc::from(source_artifact_index.digest()),
            bundle_schema_version: Arc::from(bundle_schema_identity.schema_version()),
            bundle_digest_algorithm: Arc::from(bundle_schema_identity.digest_algorithm()),
            semantic_source_digests,
            selected_record_count,
            expected_field_count,
            counters: BridgeSubscriptionCertificationCounterSnapshot::combine([
                *source_artifact_index.counters(),
                BridgeSubscriptionCertificationCounterSnapshot::from_assembly_plan(),
            ]),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-assembly-plan:sha256:{digest:x}"
            )),
        }
    }

    pub fn manifest_digest(&self) -> &str {
        self.manifest_digest.as_ref()
    }

    pub fn source_artifact_index_digest(&self) -> &str {
        self.source_artifact_index_digest.as_ref()
    }

    pub fn bundle_schema_version(&self) -> &str {
        self.bundle_schema_version.as_ref()
    }

    pub fn bundle_digest_algorithm(&self) -> &str {
        self.bundle_digest_algorithm.as_ref()
    }

    pub fn semantic_source_digests(
        &self,
    ) -> &BridgeSubscriptionCertificationSemanticSourceDigestSet {
        &self.semantic_source_digests
    }

    pub fn selected_record_count(&self) -> usize {
        self.selected_record_count
    }

    pub fn expected_field_count(&self) -> usize {
        self.expected_field_count
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionCertificationAssemblyRejectionKind {
    SourceArtifactBudgetExceeded,
    BundleFieldBudgetExceeded,
    ScratchCapacityTooSmall,
}

impl BridgeSubscriptionCertificationAssemblyRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceArtifactBudgetExceeded => "source_artifact_budget_exceeded",
            Self::BundleFieldBudgetExceeded => "bundle_field_budget_exceeded",
            Self::ScratchCapacityTooSmall => "scratch_capacity_too_small",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationAssemblyRejection {
    rejection_kind: BridgeSubscriptionCertificationAssemblyRejectionKind,
    assembly_plan_digest: Arc<str>,
    cost_profile_digest: Arc<str>,
    scratch_digest: Arc<str>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationAssemblyRejection {
    pub(crate) fn new(
        rejection_kind: BridgeSubscriptionCertificationAssemblyRejectionKind,
        assembly_plan_digest: impl Into<Arc<str>>,
        cost_profile_digest: impl Into<Arc<str>>,
        scratch_digest: impl Into<Arc<str>>,
        counters: BridgeSubscriptionCertificationCounterSnapshot,
    ) -> Self {
        let assembly_plan_digest = assembly_plan_digest.into();
        let cost_profile_digest = cost_profile_digest.into();
        let scratch_digest = scratch_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-assembly-rejection|kind={}|plan={assembly_plan_digest}|cost-profile={cost_profile_digest}|scratch={scratch_digest}|counters={}",
            rejection_kind.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            assembly_plan_digest,
            cost_profile_digest,
            scratch_digest,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-assembly-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionCertificationAssemblyRejectionKind {
        self.rejection_kind
    }

    pub fn assembly_plan_digest(&self) -> &str {
        self.assembly_plan_digest.as_ref()
    }

    pub fn cost_profile_digest(&self) -> &str {
        self.cost_profile_digest.as_ref()
    }

    pub fn scratch_digest(&self) -> &str {
        self.scratch_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
