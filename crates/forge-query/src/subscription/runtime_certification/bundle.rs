use crate::identity::hash_parts;

use super::coverage::CoverageResolutionPosture;
use super::error::{
    QuerySubscriptionRuntimeCertificationCounters, QuerySubscriptionRuntimeCertificationError,
    QuerySubscriptionRuntimeCertificationErrorKind,
};
use super::scope::QuerySubscriptionRuntimeCertificationScope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionCertificationCoverageWidth {
    admitted_row_count: usize,
    hostile_row_count: usize,
    covered_variation_axis_count: usize,
    digest: String,
}

impl SubscriptionCertificationCoverageWidth {
    fn new(
        admitted_row_count: usize,
        hostile_row_count: usize,
        covered_variation_axis_count: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_certification_coverage_width_v1".to_string(),
            format!("admitted_rows:{admitted_row_count}"),
            format!("hostile_rows:{hostile_row_count}"),
            format!("covered_variation_axes:{covered_variation_axis_count}"),
        ]);
        Self {
            admitted_row_count,
            hostile_row_count,
            covered_variation_axis_count,
            digest,
        }
    }

    pub fn admitted_row_count(&self) -> usize {
        self.admitted_row_count
    }

    pub fn hostile_row_count(&self) -> usize {
        self.hostile_row_count
    }

    pub fn covered_variation_axis_count(&self) -> usize {
        self.covered_variation_axis_count
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificationCoverageReceipt {
    coverage_resolution_posture: CoverageResolutionPosture,
    family_coverage_index_lookup_count: usize,
    covered_row_width: SubscriptionCertificationCoverageWidth,
    uncovered_variation_width: usize,
    digest: String,
}

impl CertificationCoverageReceipt {
    fn new(
        coverage_resolution_posture: CoverageResolutionPosture,
        family_coverage_index_lookup_count: usize,
        covered_row_width: SubscriptionCertificationCoverageWidth,
        uncovered_variation_width: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_certification_coverage_receipt_v1".to_string(),
            coverage_resolution_posture.as_str().to_string(),
            format!("family_coverage_index_lookup_count:{family_coverage_index_lookup_count}"),
            format!("covered_row_width:{}", covered_row_width.digest()),
            format!("uncovered_variation_width:{uncovered_variation_width}"),
        ]);
        Self {
            coverage_resolution_posture,
            family_coverage_index_lookup_count,
            covered_row_width,
            uncovered_variation_width,
            digest,
        }
    }

    pub fn coverage_resolution_posture(&self) -> &CoverageResolutionPosture {
        &self.coverage_resolution_posture
    }

    pub fn family_coverage_index_lookup_count(&self) -> usize {
        self.family_coverage_index_lookup_count
    }

    pub fn covered_row_width(&self) -> &SubscriptionCertificationCoverageWidth {
        &self.covered_row_width
    }

    pub fn uncovered_variation_width(&self) -> usize {
        self.uncovered_variation_width
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionRuntimeCertificationBundle {
    query_digest: String,
    subscription_family_digest: String,
    subscription_declaration_digest: String,
    bridge_declaration_digest: String,
    signal_strategy_digest: String,
    support_report_digest: String,
    bridge_parity_digest: String,
    diagnostic_bundle_digest: String,
    lifecycle_certification_digest: String,
    hostile_coverage_digest: String,
    family_coverage_digest: String,
    runtime_certification_bundle_digest: String,
    counter_snapshot: String,
    counters: QuerySubscriptionRuntimeCertificationCounters,
}

impl QuerySubscriptionRuntimeCertificationBundle {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn subscription_family_digest(&self) -> &str {
        &self.subscription_family_digest
    }

    pub fn subscription_declaration_digest(&self) -> &str {
        &self.subscription_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn support_report_digest(&self) -> &str {
        &self.support_report_digest
    }

    pub fn bridge_parity_digest(&self) -> &str {
        &self.bridge_parity_digest
    }

    pub fn diagnostic_bundle_digest(&self) -> &str {
        &self.diagnostic_bundle_digest
    }

    pub fn lifecycle_certification_digest(&self) -> &str {
        &self.lifecycle_certification_digest
    }

    pub fn hostile_coverage_digest(&self) -> &str {
        &self.hostile_coverage_digest
    }

    pub fn family_coverage_digest(&self) -> &str {
        &self.family_coverage_digest
    }

    pub fn runtime_certification_bundle_digest(&self) -> &str {
        &self.runtime_certification_bundle_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn counters(&self) -> &QuerySubscriptionRuntimeCertificationCounters {
        &self.counters
    }
}

fn family_coverage_index_lookup_count(posture: &CoverageResolutionPosture) -> usize {
    match posture {
        CoverageResolutionPosture::IndexedCoverageSet => 1,
        CoverageResolutionPosture::PrecomputedCoverageMatrix
        | CoverageResolutionPosture::MatrixScanDebtExplicit
        | CoverageResolutionPosture::MatrixScanDenied => 0,
    }
}

pub fn certify_query_subscription_runtime_family(
    scope: QuerySubscriptionRuntimeCertificationScope,
) -> Result<
    (
        QuerySubscriptionRuntimeCertificationBundle,
        CertificationCoverageReceipt,
    ),
    QuerySubscriptionRuntimeCertificationError,
> {
    let coverage_handle = scope.coverage_handle();
    let covered_row_width = SubscriptionCertificationCoverageWidth::new(
        coverage_handle.admitted_rows().len(),
        coverage_handle.hostile_rows().len(),
        6,
    );
    let uncovered_variation_width = usize::from(coverage_handle.admitted_rows().is_empty())
        + usize::from(coverage_handle.hostile_rows().is_empty())
        + usize::from(coverage_handle.basis_variations().digests().is_empty())
        + usize::from(coverage_handle.policy_variations().digests().is_empty())
        + usize::from(coverage_handle.tenant_variations().digests().is_empty())
        + usize::from(
            coverage_handle
                .relationship_proof_variations()
                .digests()
                .is_empty(),
        )
        + usize::from(coverage_handle.view_shape_variations().digests().is_empty())
        + usize::from(
            coverage_handle
                .lifecycle_class_variations()
                .classes()
                .is_empty(),
        );

    if coverage_handle.hostile_rows().is_empty() {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::MissingHostileCoverage,
            "runtime family certification requires at least one hostile family coverage row for every supported family",
            &[
                format!("family:{}", scope.family().as_str()),
                format!("coverage:{}", coverage_handle.family_coverage_digest()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::uncovered_family(false),
        ));
    }

    if uncovered_variation_width > 0 {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::UncoveredFamily,
            "runtime family certification requires admitted coverage plus non-empty basis, policy, tenant, relationship-proof, view-shape, and lifecycle variation sets",
            &[
                format!("family:{}", scope.family().as_str()),
                format!("coverage:{}", coverage_handle.family_coverage_digest()),
                format!("uncovered_variation_width:{uncovered_variation_width}"),
            ],
            QuerySubscriptionRuntimeCertificationCounters::uncovered_family(false),
        ));
    }

    let hostile_coverage_digest = hash_parts(
        &std::iter::once("query_subscription_runtime_hostile_coverage_v1".to_string())
            .chain(
                coverage_handle
                    .hostile_rows()
                    .iter()
                    .map(|row| row.row_digest().to_string()),
            )
            .collect::<Vec<_>>(),
    );
    let coverage_resolution_posture = *coverage_handle.coverage_resolution_posture();
    let receipt = CertificationCoverageReceipt::new(
        coverage_resolution_posture,
        family_coverage_index_lookup_count(&coverage_resolution_posture),
        covered_row_width,
        uncovered_variation_width,
    );
    let counters = QuerySubscriptionRuntimeCertificationCounters::certified(
        coverage_handle.hostile_rows().len(),
        coverage_resolution_posture,
    );
    let counter_snapshot = counters.digest();
    let runtime_certification_bundle_digest = hash_parts(&[
        "query_subscription_runtime_certification_bundle_v1".to_string(),
        scope.scope_digest().to_string(),
        scope.support_report().report_digest().to_string(),
        scope.bridge_parity().explanation_digest().to_string(),
        scope
            .admitted_diagnostic_bundle()
            .bundle_digest()
            .to_string(),
        scope
            .lifecycle_certification()
            .certification_bundle_for_reporting()
            .to_string(),
        coverage_handle.family_coverage_digest().to_string(),
        hostile_coverage_digest.clone(),
        receipt.digest().to_string(),
        counter_snapshot.clone(),
    ]);

    Ok((
        QuerySubscriptionRuntimeCertificationBundle {
            query_digest: scope.lifecycle_certification().query_digest().to_string(),
            subscription_family_digest: scope
                .lifecycle_certification()
                .subscription_family_for_reporting()
                .to_string(),
            subscription_declaration_digest: scope
                .lifecycle_certification()
                .query_declaration_for_reporting()
                .to_string(),
            bridge_declaration_digest: scope
                .lifecycle_certification()
                .bridge_declaration_for_reporting()
                .to_string(),
            signal_strategy_digest: scope
                .lifecycle_certification()
                .signal_strategy_for_reporting()
                .to_string(),
            support_report_digest: scope.support_report().report_digest().to_string(),
            bridge_parity_digest: scope.bridge_parity().explanation_digest().to_string(),
            diagnostic_bundle_digest: scope
                .admitted_diagnostic_bundle()
                .bundle_digest()
                .to_string(),
            lifecycle_certification_digest: scope
                .lifecycle_certification()
                .certification_bundle_for_reporting()
                .to_string(),
            hostile_coverage_digest,
            family_coverage_digest: coverage_handle.family_coverage_digest().to_string(),
            runtime_certification_bundle_digest,
            counter_snapshot,
            counters,
        },
        receipt,
    ))
}
