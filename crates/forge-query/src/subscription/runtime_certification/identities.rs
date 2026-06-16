use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::coverage::{CoverageResolutionPosture, QuerySubscriptionFamilyCoverageRow};
use super::error::QuerySubscriptionRuntimeCertificationCounters;

#[allow(clippy::too_many_arguments)]
pub(super) fn coverage_row_identity(
    family: &str,
    row_class: &str,
    query_scope_identity: &ForgeQueryEvidenceIdentity,
    subscription_family_identity: &ForgeQueryEvidenceIdentity,
    subscription_declaration_identity: &ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    basis_identity: &ForgeQueryEvidenceIdentity,
    policy_identity: &ForgeQueryEvidenceIdentity,
    tenant_basis_identity: &ForgeQueryEvidenceIdentity,
    relationship_proof_identity: &ForgeQueryEvidenceIdentity,
    view_shape_identity: &ForgeQueryEvidenceIdentity,
    support_report_identity: &ForgeQueryEvidenceIdentity,
    bridge_parity_identity: &ForgeQueryEvidenceIdentity,
    lifecycle_certification_identity: &ForgeQueryEvidenceIdentity,
    diagnostic_bundle_identity: &ForgeQueryEvidenceIdentity,
    lifecycle_class: &str,
    failure_identity: Option<&ForgeQueryEvidenceIdentity>,
) -> ForgeQueryEvidenceIdentity {
    let mut composer = ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_family_coverage_row_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_shape(ForgeQueryEvidenceTag::new("row_class"), row_class)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("query"), query_scope_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_family"),
            subscription_family_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_declaration"),
            subscription_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis"), basis_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("policy"), policy_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("tenant_basis"),
            tenant_basis_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("relationship_proof"),
            relationship_proof_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("view_shape"),
            view_shape_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("support_report"),
            support_report_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_parity"),
            bridge_parity_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("lifecycle_certification"),
            lifecycle_certification_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("diagnostic_bundle"),
            diagnostic_bundle_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle_class"),
            lifecycle_class,
        );
    if let Some(failure_identity) = failure_identity {
        composer = composer.field_evidence_identity(
            ForgeQueryEvidenceTag::new("failure"),
            failure_identity,
        );
    } else {
        composer = composer.field_shape(ForgeQueryEvidenceTag::new("failure"), "none");
    }
    composer.seal()
}

pub(super) fn coverage_matrix_identity<'a, I>(rows: I) -> ForgeQueryEvidenceIdentity
where
    I: IntoIterator<Item = &'a ForgeQueryEvidenceIdentity>,
{
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_family_coverage_matrix_v1",
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("row"), rows)
        .seal()
}

pub(super) fn coverage_evidence_variation_set_identity<'a, I>(
    identity_family: &str,
    values: I,
) -> ForgeQueryEvidenceIdentity
where
    I: IntoIterator<Item = &'a ForgeQueryEvidenceIdentity>,
{
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            identity_family,
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("variation"), values)
        .seal()
}

pub(super) fn lifecycle_class_variation_set_identity<'a, I>(
    classes: I,
) -> ForgeQueryEvidenceIdentity
where
    I: IntoIterator<Item = &'a str>,
{
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_lifecycle_class_variation_set_v1",
        )
        .field_value_sequence(ForgeQueryEvidenceTag::new("class"), classes)
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn certified_family_coverage_handle_identity(
    family: &str,
    posture: &str,
    matrix_identity: &ForgeQueryEvidenceIdentity,
    basis_variation_identity: &ForgeQueryEvidenceIdentity,
    policy_variation_identity: &ForgeQueryEvidenceIdentity,
    tenant_variation_identity: &ForgeQueryEvidenceIdentity,
    relationship_proof_variation_identity: &ForgeQueryEvidenceIdentity,
    view_shape_variation_identity: &ForgeQueryEvidenceIdentity,
    lifecycle_class_variation_identity: &ForgeQueryEvidenceIdentity,
    admitted_row_count: usize,
    hostile_row_count: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_certified_family_coverage_handle_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("matrix"), matrix_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_variations"),
            basis_variation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("policy_variations"),
            policy_variation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("tenant_variations"),
            tenant_variation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("relationship_proof_variations"),
            relationship_proof_variation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("view_shape_variations"),
            view_shape_variation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("lifecycle_class_variations"),
            lifecycle_class_variation_identity,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("admitted_row_count"),
            admitted_row_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("hostile_row_count"),
            hostile_row_count,
        )
        .seal()
}

pub(super) fn coverage_width_identity(
    admitted_row_count: usize,
    hostile_row_count: usize,
    covered_variation_axis_count: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_certification_coverage_width_v1",
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("admitted_rows"),
            admitted_row_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("hostile_rows"),
            hostile_row_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("covered_variation_axes"),
            covered_variation_axis_count,
        )
        .seal()
}

pub(super) fn coverage_receipt_identity(
    coverage_resolution_posture: CoverageResolutionPosture,
    family_coverage_index_lookup_count: usize,
    covered_row_width: &ForgeQueryEvidenceIdentity,
    uncovered_variation_width: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_certification_coverage_receipt_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("coverage_resolution_posture"),
            coverage_resolution_posture.as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("family_coverage_index_lookup_count"),
            family_coverage_index_lookup_count,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("covered_row_width"),
            covered_row_width,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("uncovered_variation_width"),
            uncovered_variation_width,
        )
        .seal()
}

pub(super) fn hostile_coverage_identity(
    hostile_rows: &[QuerySubscriptionFamilyCoverageRow],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_runtime_hostile_coverage_v1",
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("hostile_row"),
            hostile_rows.iter().map(|row| row.row_identity()),
        )
        .seal()
}

pub(super) fn runtime_certification_bundle_identity(
    scope_identity: &ForgeQueryEvidenceIdentity,
    support_report_identity: &ForgeQueryEvidenceIdentity,
    bridge_parity_identity: &ForgeQueryEvidenceIdentity,
    diagnostic_bundle_identity: &ForgeQueryEvidenceIdentity,
    lifecycle_certification_identity: &ForgeQueryEvidenceIdentity,
    family_coverage_identity: &ForgeQueryEvidenceIdentity,
    hostile_coverage_identity: &ForgeQueryEvidenceIdentity,
    coverage_receipt_identity: &ForgeQueryEvidenceIdentity,
    counter_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_runtime_certification_bundle_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("scope"), scope_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("support_report"),
            support_report_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_parity"),
            bridge_parity_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("diagnostic_bundle"),
            diagnostic_bundle_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("lifecycle_certification"),
            lifecycle_certification_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("family_coverage"),
            family_coverage_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("hostile_coverage"),
            hostile_coverage_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("coverage_receipt"),
            coverage_receipt_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counters"), counter_identity)
        .seal()
}

pub(super) fn runtime_certification_counter_identity(
    counters: &QuerySubscriptionRuntimeCertificationCounters,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_runtime_certification_counters_v1",
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("certification_scope_emission_count"),
            counters.certification_scope_emission_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("certified_family_count"),
            counters.certified_family_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("hostile_row_coverage_count"),
            counters.hostile_row_coverage_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("uncovered_family_denial_count"),
            counters.uncovered_family_denial_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("family_coverage_index_lookup_count"),
            counters.family_coverage_index_lookup_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("family_coverage_matrix_scan_debt_count"),
            counters.family_coverage_matrix_scan_debt_count() as usize,
        )
        .seal()
}
