use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationAssemblyPlan, BridgeSubscriptionCertificationBundleDraft,
    BridgeSubscriptionCertificationBundleSchemaIdentity,
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationFieldExpectation, BridgeSubscriptionCertificationScratch,
    BridgeSubscriptionReferenceWorkloadComponentIdSet,
    BridgeSubscriptionReferenceWorkloadLaneIdSet, BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionReferenceWorkloadManifestSealed,
    BridgeSubscriptionReferenceWorkloadProductIdSet, BridgeSubscriptionSourceArtifactEvidence,
    BridgeSubscriptionSourceArtifactIndex, BridgeSubscriptionSourceArtifactInput,
    BridgeSubscriptionSourceArtifactKind, BridgeSubscriptionSourceArtifactRole,
    BridgeSubscriptionSourceArtifactScenario,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationSchemaParityReport {
    parity_bundle_digest: Arc<str>,
    divergent_bundle_digest: Arc<str>,
    parity_schema_version: Arc<str>,
    divergent_schema_version: Arc<str>,
    parity_digest_algorithm: Arc<str>,
    divergent_digest_algorithm: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    suppressed_failure_boundary_count: usize,
    semantic_drift_shadowed_by_schema_divergence: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationSchemaParityReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let parity = assemble_bundle(
            &manifest,
            BridgeSubscriptionSourceArtifactRole::Parity,
            BridgeSubscriptionCertificationBundleSchemaIdentity::Current,
        );
        let semantically_different = assemble_bundle(
            &manifest,
            BridgeSubscriptionSourceArtifactRole::Divergent,
            BridgeSubscriptionCertificationBundleSchemaIdentity::DivergentSchemaParityIdentity,
        );
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::BundleSchemaOrDigestDivergence),
            None,
        )
        .expect("schema parity certification names its expected boundary");
        let comparison = BridgeSubscriptionCertificationComparisonReport::compare(
            plan,
            &parity,
            &semantically_different,
        );
        Self::from_certified_parts(parity, semantically_different, comparison)
    }

    fn from_certified_parts(
        parity: BridgeSubscriptionCertificationBundleSealed,
        divergent: BridgeSubscriptionCertificationBundleSealed,
        comparison: BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("schema divergence comparison must localize a primary failure");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("schema divergence comparison must expose precedence");
        let suppressed_failure_boundary_count = comparison.suppressed_failure_boundaries().len();
        let semantic_drift_shadowed_by_schema_divergence = parity
            .semantic_digests()
            .subscription_registry_digest()
            != divergent.semantic_digests().subscription_registry_digest()
            && comparison.mismatch_count() == 1
            && primary_failure_boundary
                == BridgeSubscriptionCertificationFailureBoundary::BundleSchemaOrDigestDivergence
            && primary_failure_precedence_stage
                == BridgeSubscriptionCertificationFailurePrecedenceStage::BundleSchemaParity
            && suppressed_failure_boundary_count == 0;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_schema_parity_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-schema-parity-report|parity={}|divergent={}|parity-schema={}|divergent-schema={}|parity-algorithm={}|divergent-algorithm={}|comparison={}|primary={}|stage={}|suppressed={suppressed_failure_boundary_count}|semantic-short-circuit={semantic_drift_shadowed_by_schema_divergence}|counters={}",
            parity.digest(),
            divergent.digest(),
            parity.schema_version(),
            divergent.schema_version(),
            parity.digest_algorithm(),
            divergent.digest_algorithm(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            parity_bundle_digest: Arc::from(parity.digest()),
            divergent_bundle_digest: Arc::from(divergent.digest()),
            parity_schema_version: Arc::from(parity.schema_version()),
            divergent_schema_version: Arc::from(divergent.schema_version()),
            parity_digest_algorithm: Arc::from(parity.digest_algorithm()),
            divergent_digest_algorithm: Arc::from(divergent.digest_algorithm()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            suppressed_failure_boundary_count,
            semantic_drift_shadowed_by_schema_divergence,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-schema-parity-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn parity_bundle_digest(&self) -> &str {
        self.parity_bundle_digest.as_ref()
    }

    pub fn divergent_bundle_digest(&self) -> &str {
        self.divergent_bundle_digest.as_ref()
    }

    pub fn parity_schema_version(&self) -> &str {
        self.parity_schema_version.as_ref()
    }

    pub fn divergent_schema_version(&self) -> &str {
        self.divergent_schema_version.as_ref()
    }

    pub fn parity_digest_algorithm(&self) -> &str {
        self.parity_digest_algorithm.as_ref()
    }

    pub fn divergent_digest_algorithm(&self) -> &str {
        self.divergent_digest_algorithm.as_ref()
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

    pub fn semantic_drift_shadowed_by_schema_divergence(&self) -> bool {
        self.semantic_drift_shadowed_by_schema_divergence
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
        BridgeSubscriptionReferenceWorkloadProductIdSet::from_declared_product_labels(
            (0..128).map(|slot| format!("product-{slot:03}")),
        ),
        BridgeSubscriptionReferenceWorkloadComponentIdSet::from_declared_component_labels([
            "steel", "rubber", "copper", "glass", "labor",
        ]),
        BridgeSubscriptionReferenceWorkloadLaneIdSet::from_declared_lane_labels([
            "authoritative-live",
            "historical-replay",
            "branch-local",
            "preview-discard",
        ]),
    )
    .seal()
    .expect("schema parity fixture manifest should seal")
}

fn assemble_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    declaration_role: BridgeSubscriptionSourceArtifactRole,
    schema_identity: BridgeSubscriptionCertificationBundleSchemaIdentity,
) -> BridgeSubscriptionCertificationBundleSealed {
    let index = BridgeSubscriptionSourceArtifactIndex::build(vec![
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            declaration_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
    ]);
    let plan = BridgeSubscriptionCertificationAssemblyPlan::plan_with_bundle_identity(
        manifest,
        &index,
        BridgeSubscriptionCertificationFieldExpectation::CompleteReferenceBundle,
        schema_identity,
    );
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        8,
        16,
        32,
        false,
    )
    .expect("schema parity sparse cost profile should admit");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("schema parity bundle should assemble")
        .seal()
}

fn source_artifact(
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    role: BridgeSubscriptionSourceArtifactRole,
) -> BridgeSubscriptionSourceArtifactInput {
    BridgeSubscriptionSourceArtifactInput::from_evidence(
        BridgeSubscriptionSourceArtifactEvidence::scenario(
            artifact_kind,
            BridgeSubscriptionSourceArtifactScenario::SchemaParity,
            role,
        ),
    )
}
