use std::collections::BTreeSet;

use crate::identity::hash_parts;

use super::super::bridge_parity::QuerySubscriptionBridgeParityExplanation;
use super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::diagnostic::QuerySubscriptionDiagnosticStage;
use super::super::diagnostic::{
    QuerySubscriptionAdmittedDiagnosticBundle, QuerySubscriptionDeniedDiagnosticBundle,
    QuerySubscriptionDiagnosticFailure,
};
use super::super::family::QuerySubscriptionFamily;
use super::super::support::QuerySubscriptionSupportReport;
use super::super::support::{QuerySubscriptionSupportClass, QuerySubscriptionSupportPosture};
use super::error::{
    QuerySubscriptionRuntimeCertificationCounters, QuerySubscriptionRuntimeCertificationError,
    QuerySubscriptionRuntimeCertificationErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverageResolutionPosture {
    IndexedCoverageSet,
    PrecomputedCoverageMatrix,
    MatrixScanDebtExplicit,
    MatrixScanDenied,
}

impl CoverageResolutionPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndexedCoverageSet => "indexed_coverage_set",
            Self::PrecomputedCoverageMatrix => "precomputed_coverage_matrix",
            Self::MatrixScanDebtExplicit => "matrix_scan_debt_explicit",
            Self::MatrixScanDenied => "matrix_scan_denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum QuerySubscriptionLifecycleCoverageClass {
    ActiveLifecycle,
    Continuation,
    PreviewIsolation,
    LifecycleCloseout,
}

impl QuerySubscriptionLifecycleCoverageClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActiveLifecycle => "active_lifecycle",
            Self::Continuation => "continuation",
            Self::PreviewIsolation => "preview_isolation",
            Self::LifecycleCloseout => "lifecycle_closeout",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionFamilyCoverageRowClass {
    Admitted,
    HostileDenied,
}

impl QuerySubscriptionFamilyCoverageRowClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::HostileDenied => "hostile_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionFamilyCoverageRow {
    family: QuerySubscriptionFamily,
    row_class: QuerySubscriptionFamilyCoverageRowClass,
    query_digest: String,
    subscription_family_digest: String,
    subscription_declaration_digest: String,
    bridge_declaration_digest: String,
    signal_strategy_digest: String,
    support_report_digest: String,
    bridge_parity_digest: String,
    lifecycle_certification_digest: String,
    diagnostic_bundle_digest: String,
    failure_digest: Option<String>,
    basis_digest: String,
    policy_digest: String,
    tenant_basis_digest: String,
    relationship_proof_digest: String,
    view_shape_digest: String,
    lifecycle_class: QuerySubscriptionLifecycleCoverageClass,
    row_digest: String,
}

impl QuerySubscriptionFamilyCoverageRow {
    pub fn admitted(
        family: &QuerySubscriptionFamily,
        support: &QuerySubscriptionSupportReport,
        parity: &QuerySubscriptionBridgeParityExplanation,
        lifecycle: &SubscriptionLifecycleCertificationBundle,
        diagnostic: &QuerySubscriptionAdmittedDiagnosticBundle,
        lifecycle_class: QuerySubscriptionLifecycleCoverageClass,
    ) -> Result<Self, QuerySubscriptionRuntimeCertificationError> {
        validate_row_alignment(family, support, parity, lifecycle)?;
        if diagnostic.support_report_digest() != support.report_digest()
            || diagnostic.lifecycle_certification_digest()
                != lifecycle.certification_bundle_for_reporting()
        {
            return Err(QuerySubscriptionRuntimeCertificationError::new(
                QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch,
                "admitted family coverage rows require diagnostic bundle digests aligned with support and lifecycle evidence",
                &[
                    format!("diagnostic_support:{}", diagnostic.support_report_digest()),
                    format!("support:{}", support.report_digest()),
                    format!(
                        "diagnostic_lifecycle:{}",
                        diagnostic.lifecycle_certification_digest()
                    ),
                    format!("lifecycle:{}", lifecycle.certification_bundle_for_reporting()),
                ],
                QuerySubscriptionRuntimeCertificationCounters::default(),
            ));
        }

        Ok(Self::new(
            family,
            QuerySubscriptionFamilyCoverageRowClass::Admitted,
            lifecycle,
            support.report_digest(),
            parity.explanation_digest(),
            diagnostic.bundle_digest(),
            None,
            lifecycle_class,
        ))
    }

    pub fn hostile(
        family: &QuerySubscriptionFamily,
        support: &QuerySubscriptionSupportReport,
        parity: &QuerySubscriptionBridgeParityExplanation,
        lifecycle: &SubscriptionLifecycleCertificationBundle,
        diagnostic: &QuerySubscriptionDeniedDiagnosticBundle,
        failure: &QuerySubscriptionDiagnosticFailure,
        lifecycle_class: QuerySubscriptionLifecycleCoverageClass,
    ) -> Result<Self, QuerySubscriptionRuntimeCertificationError> {
        validate_row_alignment(family, support, parity, lifecycle)?;
        validate_hostile_diagnostic_alignment(diagnostic, support, lifecycle)?;
        if diagnostic.semantic_labels().query_family_label() != family.as_str() {
            return Err(QuerySubscriptionRuntimeCertificationError::new(
                QuerySubscriptionRuntimeCertificationErrorKind::CoverageFamilyMismatch,
                "hostile family coverage rows require denied diagnostic bundles for the same query subscription family",
                &[
                    format!(
                        "diagnostic_family:{}",
                        diagnostic.semantic_labels().query_family_label()
                    ),
                    format!("expected_family:{}", family.as_str()),
                ],
                QuerySubscriptionRuntimeCertificationCounters::default(),
            ));
        }

        Ok(Self::new(
            family,
            QuerySubscriptionFamilyCoverageRowClass::HostileDenied,
            lifecycle,
            support.report_digest(),
            parity.explanation_digest(),
            diagnostic.bundle_digest(),
            Some(failure.failure_digest()),
            lifecycle_class,
        ))
    }

    fn new(
        family: &QuerySubscriptionFamily,
        row_class: QuerySubscriptionFamilyCoverageRowClass,
        lifecycle: &SubscriptionLifecycleCertificationBundle,
        support_report_digest: &str,
        bridge_parity_digest: &str,
        diagnostic_bundle_digest: &str,
        failure_digest: Option<&str>,
        lifecycle_class: QuerySubscriptionLifecycleCoverageClass,
    ) -> Self {
        let row_digest = hash_parts(&[
            "query_subscription_family_coverage_row_v1".to_string(),
            family.as_str().to_string(),
            row_class.as_str().to_string(),
            lifecycle.query_digest().to_string(),
            lifecycle.subscription_family_digest().to_string(),
            lifecycle.query_declaration_for_reporting().to_string(),
            lifecycle.bridge_declaration_for_reporting().to_string(),
            lifecycle.signal_strategy_for_reporting().to_string(),
            lifecycle.basis_digest().to_string(),
            lifecycle.policy_digest().to_string(),
            lifecycle.tenant_basis_digest().to_string(),
            lifecycle.relationship_proof_digest().to_string(),
            lifecycle.view_shape_digest().to_string(),
            support_report_digest.to_string(),
            bridge_parity_digest.to_string(),
            lifecycle.certification_bundle_for_reporting().to_string(),
            diagnostic_bundle_digest.to_string(),
            lifecycle_class.as_str().to_string(),
            format!("failure:{}", failure_digest.unwrap_or("none")),
        ]);
        Self {
            family: family.clone(),
            row_class,
            query_digest: lifecycle.query_digest().to_string(),
            subscription_family_digest: lifecycle.subscription_family_digest().to_string(),
            subscription_declaration_digest: lifecycle
                .query_declaration_for_reporting()
                .to_string(),
            bridge_declaration_digest: lifecycle.bridge_declaration_for_reporting().to_string(),
            signal_strategy_digest: lifecycle.signal_strategy_for_reporting().to_string(),
            support_report_digest: support_report_digest.to_string(),
            bridge_parity_digest: bridge_parity_digest.to_string(),
            lifecycle_certification_digest: lifecycle.certification_bundle_for_reporting().to_string(),
            diagnostic_bundle_digest: diagnostic_bundle_digest.to_string(),
            failure_digest: failure_digest.map(ToOwned::to_owned),
            basis_digest: lifecycle.basis_digest().to_string(),
            policy_digest: lifecycle.policy_digest().to_string(),
            tenant_basis_digest: lifecycle.tenant_basis_digest().to_string(),
            relationship_proof_digest: lifecycle.relationship_proof_digest().to_string(),
            view_shape_digest: lifecycle.view_shape_digest().to_string(),
            lifecycle_class,
            row_digest,
        }
    }

    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn row_class(&self) -> &QuerySubscriptionFamilyCoverageRowClass {
        &self.row_class
    }

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

    pub fn lifecycle_certification_digest(&self) -> &str {
        &self.lifecycle_certification_digest
    }

    pub fn diagnostic_bundle_digest(&self) -> &str {
        &self.diagnostic_bundle_digest
    }

    pub fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_basis_digest(&self) -> &str {
        &self.tenant_basis_digest
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn view_shape_digest(&self) -> &str {
        &self.view_shape_digest
    }

    pub fn lifecycle_class(&self) -> &QuerySubscriptionLifecycleCoverageClass {
        &self.lifecycle_class
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn validate_hostile_diagnostic_alignment(
    diagnostic: &QuerySubscriptionDeniedDiagnosticBundle,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
) -> Result<(), QuerySubscriptionRuntimeCertificationError> {
    if let Some(support_report_digest) = diagnostic.support_report_digest() {
        if support_report_digest != support.report_digest() {
            return Err(QuerySubscriptionRuntimeCertificationError::new(
                QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch,
                "hostile family coverage rows require denied diagnostic bundles to preserve support-report identity when support evidence is present",
                &[
                    format!("diagnostic_support:{support_report_digest}"),
                    format!("support:{}", support.report_digest()),
                    format!("diagnostic:{}", diagnostic.bundle_digest()),
                ],
                QuerySubscriptionRuntimeCertificationCounters::default(),
            ));
        }
    }

    validate_trace_stage_source(
        diagnostic,
        QuerySubscriptionDiagnosticStage::Declaration,
        lifecycle.query_declaration_for_reporting(),
        "hostile family coverage rows require denied diagnostic traces to preserve declaration identity",
    )?;
    validate_trace_stage_source(
        diagnostic,
        QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
        lifecycle.bridge_declaration_for_reporting(),
        "hostile family coverage rows require denied diagnostic traces to preserve bridge declaration identity",
    )?;
    validate_trace_stage_source(
        diagnostic,
        QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
        lifecycle.admission_for_reporting(),
        "hostile family coverage rows require denied diagnostic traces to preserve runtime admission identity",
    )?;

    Ok(())
}

fn validate_trace_stage_source(
    diagnostic: &QuerySubscriptionDeniedDiagnosticBundle,
    stage: QuerySubscriptionDiagnosticStage,
    expected_source_digest: &str,
    message: &'static str,
) -> Result<(), QuerySubscriptionRuntimeCertificationError> {
    let Some(stage_trace) = diagnostic
        .trace()
        .stage_traces()
        .iter()
        .find(|stage_trace| stage_trace.stage() == &stage)
    else {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch,
            message,
            &[
                format!("diagnostic:{}", diagnostic.bundle_digest()),
                format!("missing_stage:{}", stage.as_str()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    };

    if stage_trace.source_digest() != expected_source_digest {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch,
            message,
            &[
                format!("diagnostic:{}", diagnostic.bundle_digest()),
                format!("stage:{}", stage.as_str()),
                format!("trace_source:{}", stage_trace.source_digest()),
                format!("expected_source:{expected_source_digest}"),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBasisVariationSet {
    digests: Vec<String>,
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionPolicyVariationSet {
    digests: Vec<String>,
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionTenantVariationSet {
    digests: Vec<String>,
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionRelationshipProofVariationSet {
    digests: Vec<String>,
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionViewShapeVariationSet {
    digests: Vec<String>,
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionLifecycleClassVariationSet {
    classes: Vec<QuerySubscriptionLifecycleCoverageClass>,
    digest: String,
}

macro_rules! variation_set_impl {
    ($name:ident, $prefix:literal) => {
        impl $name {
            fn from_set(values: BTreeSet<String>) -> Self {
                let digests = values.into_iter().collect::<Vec<_>>();
                let digest = hash_parts(
                    &std::iter::once($prefix.to_string())
                        .chain(digests.iter().cloned())
                        .collect::<Vec<_>>(),
                );
                Self { digests, digest }
            }

            pub fn digests(&self) -> &[String] {
                &self.digests
            }

            pub fn digest(&self) -> &str {
                &self.digest
            }
        }
    };
}

variation_set_impl!(
    QuerySubscriptionBasisVariationSet,
    "query_subscription_basis_variation_set_v1"
);
variation_set_impl!(
    QuerySubscriptionPolicyVariationSet,
    "query_subscription_policy_variation_set_v1"
);
variation_set_impl!(
    QuerySubscriptionTenantVariationSet,
    "query_subscription_tenant_variation_set_v1"
);
variation_set_impl!(
    QuerySubscriptionRelationshipProofVariationSet,
    "query_subscription_relationship_proof_variation_set_v1"
);
variation_set_impl!(
    QuerySubscriptionViewShapeVariationSet,
    "query_subscription_view_shape_variation_set_v1"
);

impl QuerySubscriptionLifecycleClassVariationSet {
    fn from_set(values: BTreeSet<QuerySubscriptionLifecycleCoverageClass>) -> Self {
        let classes = values.into_iter().collect::<Vec<_>>();
        let digest = hash_parts(
            &std::iter::once("query_subscription_lifecycle_class_variation_set_v1".to_string())
                .chain(classes.iter().map(|value| value.as_str().to_string()))
                .collect::<Vec<_>>(),
        );
        Self { classes, digest }
    }

    pub fn classes(&self) -> &[QuerySubscriptionLifecycleCoverageClass] {
        &self.classes
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionFamilyCoverageMatrix {
    rows: Vec<QuerySubscriptionFamilyCoverageRow>,
    family_coverage_digest: String,
}

impl QuerySubscriptionFamilyCoverageMatrix {
    pub fn rows(&self) -> &[QuerySubscriptionFamilyCoverageRow] {
        &self.rows
    }

    pub fn family_coverage_digest(&self) -> &str {
        &self.family_coverage_digest
    }
}

pub fn build_query_subscription_family_coverage_matrix(
    rows: Vec<QuerySubscriptionFamilyCoverageRow>,
) -> QuerySubscriptionFamilyCoverageMatrix {
    let mut digest_parts = vec!["query_subscription_family_coverage_matrix_v1".to_string()];
    digest_parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
    digest_parts.sort();
    QuerySubscriptionFamilyCoverageMatrix {
        rows,
        family_coverage_digest: hash_parts(&digest_parts),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedFamilyCoverageHandle {
    family: QuerySubscriptionFamily,
    coverage_resolution_posture: CoverageResolutionPosture,
    admitted_rows: Vec<QuerySubscriptionFamilyCoverageRow>,
    hostile_rows: Vec<QuerySubscriptionFamilyCoverageRow>,
    basis_variations: QuerySubscriptionBasisVariationSet,
    policy_variations: QuerySubscriptionPolicyVariationSet,
    tenant_variations: QuerySubscriptionTenantVariationSet,
    relationship_proof_variations: QuerySubscriptionRelationshipProofVariationSet,
    view_shape_variations: QuerySubscriptionViewShapeVariationSet,
    lifecycle_class_variations: QuerySubscriptionLifecycleClassVariationSet,
    family_coverage_digest: String,
}

impl CertifiedFamilyCoverageHandle {
    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn coverage_resolution_posture(&self) -> &CoverageResolutionPosture {
        &self.coverage_resolution_posture
    }

    pub fn admitted_rows(&self) -> &[QuerySubscriptionFamilyCoverageRow] {
        &self.admitted_rows
    }

    pub fn hostile_rows(&self) -> &[QuerySubscriptionFamilyCoverageRow] {
        &self.hostile_rows
    }

    pub fn basis_variations(&self) -> &QuerySubscriptionBasisVariationSet {
        &self.basis_variations
    }

    pub fn policy_variations(&self) -> &QuerySubscriptionPolicyVariationSet {
        &self.policy_variations
    }

    pub fn tenant_variations(&self) -> &QuerySubscriptionTenantVariationSet {
        &self.tenant_variations
    }

    pub fn relationship_proof_variations(&self) -> &QuerySubscriptionRelationshipProofVariationSet {
        &self.relationship_proof_variations
    }

    pub fn view_shape_variations(&self) -> &QuerySubscriptionViewShapeVariationSet {
        &self.view_shape_variations
    }

    pub fn lifecycle_class_variations(&self) -> &QuerySubscriptionLifecycleClassVariationSet {
        &self.lifecycle_class_variations
    }

    pub fn family_coverage_digest(&self) -> &str {
        &self.family_coverage_digest
    }
}

pub fn build_certified_family_coverage_handle(
    matrix: &QuerySubscriptionFamilyCoverageMatrix,
    family: &QuerySubscriptionFamily,
    posture: CoverageResolutionPosture,
) -> Result<CertifiedFamilyCoverageHandle, QuerySubscriptionRuntimeCertificationError> {
    if posture == CoverageResolutionPosture::MatrixScanDenied {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CoverageResolutionDenied,
            "runtime family coverage handle construction may not proceed from a denied matrix-scan posture",
            &[format!("family:{}", family.as_str())],
            QuerySubscriptionRuntimeCertificationCounters::uncovered_family(false),
        ));
    }

    let grouped_rows = matrix
        .rows()
        .iter()
        .filter(|row| row.family() == family)
        .cloned()
        .collect::<Vec<_>>();
    if grouped_rows.is_empty() {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CoverageFamilyMissing,
            "runtime family coverage handle construction requires at least one family-scoped coverage row",
            &[
                format!("family:{}", family.as_str()),
                format!("matrix:{}", matrix.family_coverage_digest()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::uncovered_family(
                posture == CoverageResolutionPosture::MatrixScanDebtExplicit,
            ),
        ));
    }

    let mut admitted_rows = Vec::new();
    let mut hostile_rows = Vec::new();
    let mut basis_digests = BTreeSet::new();
    let mut policy_digests = BTreeSet::new();
    let mut tenant_digests = BTreeSet::new();
    let mut relationship_digests = BTreeSet::new();
    let mut view_shape_digests = BTreeSet::new();
    let mut lifecycle_classes = BTreeSet::new();

    for row in grouped_rows {
        basis_digests.insert(row.basis_digest().to_string());
        policy_digests.insert(row.policy_digest().to_string());
        tenant_digests.insert(row.tenant_basis_digest().to_string());
        relationship_digests.insert(row.relationship_proof_digest().to_string());
        view_shape_digests.insert(row.view_shape_digest().to_string());
        lifecycle_classes.insert(*row.lifecycle_class());
        match row.row_class() {
            QuerySubscriptionFamilyCoverageRowClass::Admitted => admitted_rows.push(row),
            QuerySubscriptionFamilyCoverageRowClass::HostileDenied => hostile_rows.push(row),
        }
    }

    let basis_variations = QuerySubscriptionBasisVariationSet::from_set(basis_digests);
    let policy_variations = QuerySubscriptionPolicyVariationSet::from_set(policy_digests);
    let tenant_variations = QuerySubscriptionTenantVariationSet::from_set(tenant_digests);
    let relationship_proof_variations =
        QuerySubscriptionRelationshipProofVariationSet::from_set(relationship_digests);
    let view_shape_variations =
        QuerySubscriptionViewShapeVariationSet::from_set(view_shape_digests);
    let lifecycle_class_variations =
        QuerySubscriptionLifecycleClassVariationSet::from_set(lifecycle_classes);

    let family_coverage_digest = hash_parts(&[
        "query_subscription_certified_family_coverage_handle_v1".to_string(),
        family.as_str().to_string(),
        posture.as_str().to_string(),
        matrix.family_coverage_digest().to_string(),
        basis_variations.digest().to_string(),
        policy_variations.digest().to_string(),
        tenant_variations.digest().to_string(),
        relationship_proof_variations.digest().to_string(),
        view_shape_variations.digest().to_string(),
        lifecycle_class_variations.digest().to_string(),
        format!("admitted_row_count:{}", admitted_rows.len()),
        format!("hostile_row_count:{}", hostile_rows.len()),
    ]);

    Ok(CertifiedFamilyCoverageHandle {
        family: family.clone(),
        coverage_resolution_posture: posture,
        admitted_rows,
        hostile_rows,
        basis_variations,
        policy_variations,
        tenant_variations,
        relationship_proof_variations,
        view_shape_variations,
        lifecycle_class_variations,
        family_coverage_digest,
    })
}

fn validate_row_alignment(
    family: &QuerySubscriptionFamily,
    support: &QuerySubscriptionSupportReport,
    parity: &QuerySubscriptionBridgeParityExplanation,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
) -> Result<(), QuerySubscriptionRuntimeCertificationError> {
    if !matches!(
        support.support_subject().support_class(),
        QuerySubscriptionSupportClass::ActiveLifecycle
            | QuerySubscriptionSupportClass::Continuation
            | QuerySubscriptionSupportClass::PreviewCloseout
    ) {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CertificationSupportClassDenied,
            "runtime family certification requires support reports from runtime-backed lifecycle, continuation, or preview-closeout phases",
            &[
                format!(
                    "support_class:{}",
                    support.support_subject().support_class().as_str()
                ),
                format!("support_report:{}", support.report_digest()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    if support.support_posture() != &QuerySubscriptionSupportPosture::RuntimeBackedCertified {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CertificationSupportPostureDenied,
            "runtime family certification requires support reports whose posture is runtime-backed certified",
            &[
                format!("support_posture:{}", support.support_posture().as_str()),
                format!("support_report:{}", support.report_digest()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    if support.support_subject().family() != family
        || parity.query_family_label() != family.as_str()
    {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CoverageFamilyMismatch,
            "runtime family coverage rows require support and bridge parity artifacts for the same query subscription family",
            &[
                format!("expected_family:{}", family.as_str()),
                format!("support_family:{}", support.support_subject().family().as_str()),
                format!("parity_family:{}", parity.query_family_label()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    if support.support_subject().declaration_digest() != lifecycle.query_declaration_for_reporting()
        || parity.comparison().query_declaration_digest()
            != lifecycle.query_declaration_for_reporting()
        || parity.comparison().bridge_declaration_digest() != lifecycle.bridge_declaration_for_reporting()
    {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch,
            "runtime family coverage rows require support, parity, and lifecycle certification to preserve canonical declaration and bridge identity",
            &[
                format!(
                    "support_declaration:{}",
                    support.support_subject().declaration_digest()
                ),
                format!("lifecycle_declaration:{}", lifecycle.query_declaration_for_reporting()),
                format!(
                    "parity_declaration:{}",
                    parity.comparison().query_declaration_digest()
                ),
                format!("parity_bridge:{}", parity.comparison().bridge_declaration_digest()),
                format!("lifecycle_bridge:{}", lifecycle.bridge_declaration_for_reporting()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    Ok(())
}
