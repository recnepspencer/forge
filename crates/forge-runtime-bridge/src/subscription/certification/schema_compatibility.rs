use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationAssemblyPlan, BridgeSubscriptionCertificationBundleDraft,
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage, BridgeSubscriptionCertificationScratch,
    BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactIndex,
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationSchemaCompatibilityReport {
    compatible_bundle_digest: Arc<str>,
    incompatible_bundle_digest: Arc<str>,
    compatible_schema_version: Arc<str>,
    incompatible_schema_version: Arc<str>,
    compatible_digest_algorithm: Arc<str>,
    incompatible_digest_algorithm: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    suppressed_failure_boundary_count: usize,
    semantic_drift_hidden_by_schema_incompatibility: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationSchemaCompatibilityReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let compatible = assemble_bundle(&manifest, "schema-compatible-declaration-v1");
        let semantically_different =
            assemble_bundle(&manifest, "schema-incompatible-declaration-v2")
                .with_schema_digest_identity_for_certification(
                    "bridge-subscription-certification-bundle-v999",
                    "sha512",
                );
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(
                BridgeSubscriptionCertificationFailureBoundary::BundleSchemaOrDigestIncompatibility,
            ),
            None,
        )
        .expect("schema compatibility certification names its expected boundary");
        let comparison = BridgeSubscriptionCertificationComparisonReport::compare(
            plan,
            &compatible,
            &semantically_different,
        );
        Self::from_certified_parts(compatible, semantically_different, comparison)
    }

    fn from_certified_parts(
        compatible: BridgeSubscriptionCertificationBundleSealed,
        incompatible: BridgeSubscriptionCertificationBundleSealed,
        comparison: BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("schema incompatibility comparison must localize a primary failure");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("schema incompatibility comparison must expose precedence");
        let suppressed_failure_boundary_count = comparison.suppressed_failure_boundaries().len();
        let semantic_drift_hidden_by_schema_incompatibility = compatible
            .semantic_digests()
            .subscription_registry_digest()
            != incompatible
                .semantic_digests()
                .subscription_registry_digest()
            && comparison.mismatch_count() == 1
            && primary_failure_boundary
                == BridgeSubscriptionCertificationFailureBoundary::BundleSchemaOrDigestIncompatibility
            && primary_failure_precedence_stage
                == BridgeSubscriptionCertificationFailurePrecedenceStage::BundleCompatibility
            && suppressed_failure_boundary_count == 0;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_schema_compatibility_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-schema-compatibility-report|compatible={}|incompatible={}|compatible-schema={}|incompatible-schema={}|compatible-algorithm={}|incompatible-algorithm={}|comparison={}|primary={}|stage={}|suppressed={suppressed_failure_boundary_count}|semantic-short-circuit={semantic_drift_hidden_by_schema_incompatibility}|counters={}",
            compatible.digest(),
            incompatible.digest(),
            compatible.schema_version(),
            incompatible.schema_version(),
            compatible.digest_algorithm(),
            incompatible.digest_algorithm(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            compatible_bundle_digest: Arc::from(compatible.digest()),
            incompatible_bundle_digest: Arc::from(incompatible.digest()),
            compatible_schema_version: Arc::from(compatible.schema_version()),
            incompatible_schema_version: Arc::from(incompatible.schema_version()),
            compatible_digest_algorithm: Arc::from(compatible.digest_algorithm()),
            incompatible_digest_algorithm: Arc::from(incompatible.digest_algorithm()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            suppressed_failure_boundary_count,
            semantic_drift_hidden_by_schema_incompatibility,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-schema-compatibility-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn compatible_bundle_digest(&self) -> &str {
        self.compatible_bundle_digest.as_ref()
    }

    pub fn incompatible_bundle_digest(&self) -> &str {
        self.incompatible_bundle_digest.as_ref()
    }

    pub fn compatible_schema_version(&self) -> &str {
        self.compatible_schema_version.as_ref()
    }

    pub fn incompatible_schema_version(&self) -> &str {
        self.incompatible_schema_version.as_ref()
    }

    pub fn compatible_digest_algorithm(&self) -> &str {
        self.compatible_digest_algorithm.as_ref()
    }

    pub fn incompatible_digest_algorithm(&self) -> &str {
        self.incompatible_digest_algorithm.as_ref()
    }

    pub fn comparison_report_digest(&self) -> &str {
        self.comparison_report_digest.as_ref()
    }

    pub fn primary_failure_boundary(&self) -> BridgeSubscriptionCertificationFailureBoundary {
        self.primary_failure_boundary
    }

    pub fn primary_failure_precedence_stage(
        &self,
    ) -> BridgeSubscriptionCertificationFailurePrecedenceStage {
        self.primary_failure_precedence_stage
    }

    pub fn suppressed_failure_boundary_count(&self) -> usize {
        self.suppressed_failure_boundary_count
    }

    pub fn semantic_drift_hidden_by_schema_incompatibility(&self) -> bool {
        self.semantic_drift_hidden_by_schema_incompatibility
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn reference_manifest() -> BridgeSubscriptionReferenceWorkloadManifestSealed {
    BridgeSubscriptionReferenceWorkloadManifestDraft::new(
        (0..128)
            .map(|slot| format!("product-{slot:03}"))
            .collect::<Vec<_>>(),
        ["steel", "rubber", "copper", "glass", "labor"].to_vec(),
        [
            "authoritative-live",
            "historical-replay",
            "branch-local",
            "preview-discard",
        ]
        .to_vec(),
    )
    .seal()
    .expect("schema compatibility fixture manifest should seal")
}

fn assemble_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    declaration_digest: &str,
) -> BridgeSubscriptionCertificationBundleSealed {
    let index = BridgeSubscriptionSourceArtifactIndex::build(vec![
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            "schema-compatibility-declaration",
            declaration_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            "schema-compatibility-admitted-subscription",
            "schema-compatibility-admitted-digest",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            "schema-compatibility-active-delivery",
            "schema-compatibility-delivery-digest",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            "schema-compatibility-strategy",
            "schema-compatibility-strategy-digest",
        ),
    ]);
    let plan = BridgeSubscriptionCertificationAssemblyPlan::plan(manifest, &index);
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        8,
        16,
        32,
        false,
    )
    .expect("schema compatibility sparse cost profile should admit");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("schema compatibility bundle should assemble")
        .seal()
}
