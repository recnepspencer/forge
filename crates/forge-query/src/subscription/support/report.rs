use crate::identity::hash_parts;

use super::super::family::QuerySubscriptionFamily;
use super::matrix::QuerySubscriptionSupportMatrix;
use super::subject::{
    QuerySubscriptionSupportClass, QuerySubscriptionSupportEvidence,
    QuerySubscriptionSupportPosture, QuerySubscriptionSupportSubject,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QuerySubscriptionSupportCounters {
    support_report_request_count: u64,
    supported_family_count: u64,
    denied_family_count: u64,
    deferred_family_count: u64,
    uncertified_family_denial_count: u64,
    support_matrix_emission_count: u64,
    support_family_index_lookup_count: u64,
    support_matrix_scan_debt_count: u64,
}

impl QuerySubscriptionSupportCounters {
    pub fn digest(&self) -> String {
        hash_parts(&[
            format!(
                "support_report_request:{}",
                self.support_report_request_count
            ),
            format!("supported_family:{}", self.supported_family_count),
            format!("denied_family:{}", self.denied_family_count),
            format!("deferred_family:{}", self.deferred_family_count),
            format!(
                "uncertified_family_denial:{}",
                self.uncertified_family_denial_count
            ),
            format!(
                "support_matrix_emission:{}",
                self.support_matrix_emission_count
            ),
            format!(
                "support_family_index_lookup:{}",
                self.support_family_index_lookup_count
            ),
            format!(
                "support_matrix_scan_debt:{}",
                self.support_matrix_scan_debt_count
            ),
        ])
    }

    pub fn support_report_request_count(&self) -> u64 {
        self.support_report_request_count
    }

    pub fn supported_family_count(&self) -> u64 {
        self.supported_family_count
    }

    pub fn denied_family_count(&self) -> u64 {
        self.denied_family_count
    }

    pub fn deferred_family_count(&self) -> u64 {
        self.deferred_family_count
    }

    pub fn uncertified_family_denial_count(&self) -> u64 {
        self.uncertified_family_denial_count
    }

    pub fn support_matrix_emission_count(&self) -> u64 {
        self.support_matrix_emission_count
    }

    pub fn support_family_index_lookup_count(&self) -> u64 {
        self.support_family_index_lookup_count
    }

    pub fn support_matrix_scan_debt_count(&self) -> u64 {
        self.support_matrix_scan_debt_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportResolutionPosture {
    IndexedFamilyLookup,
    PrecomputedFamilyMatrix,
    LinearScanDebtExplicit,
    LinearScanDenied,
}

impl SupportResolutionPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndexedFamilyLookup => "indexed_family_lookup",
            Self::PrecomputedFamilyMatrix => "precomputed_family_matrix",
            Self::LinearScanDebtExplicit => "linear_scan_debt_explicit",
            Self::LinearScanDenied => "linear_scan_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportLookupReceipt {
    family: QuerySubscriptionFamily,
    support_class: QuerySubscriptionSupportClass,
    resolution_posture: SupportResolutionPosture,
    consumed_lookup_width: usize,
    remaining_lookup_width: usize,
    digest: String,
}

impl SupportLookupReceipt {
    fn new(
        family: &QuerySubscriptionFamily,
        support_class: QuerySubscriptionSupportClass,
        resolution_posture: SupportResolutionPosture,
        consumed_lookup_width: usize,
        remaining_lookup_width: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_support_lookup_receipt_v1".to_string(),
            family.as_str().to_string(),
            support_class.as_str().to_string(),
            resolution_posture.as_str().to_string(),
            format!("consumed_lookup_width:{consumed_lookup_width}"),
            format!("remaining_lookup_width:{remaining_lookup_width}"),
        ]);
        Self {
            family: family.clone(),
            support_class,
            resolution_posture,
            consumed_lookup_width,
            remaining_lookup_width,
            digest,
        }
    }

    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn support_class(&self) -> &QuerySubscriptionSupportClass {
        &self.support_class
    }

    pub fn resolution_posture(&self) -> &SupportResolutionPosture {
        &self.resolution_posture
    }

    pub fn consumed_lookup_width(&self) -> usize {
        self.consumed_lookup_width
    }

    pub fn remaining_lookup_width(&self) -> usize {
        self.remaining_lookup_width
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionSupportReportDenialKind {
    DeclarationSourceMismatch,
    FamilySourceMismatch,
    AdmissionSourceMismatch,
    AdmissionEvidenceRequired,
}

impl QuerySubscriptionSupportReportDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeclarationSourceMismatch => "declaration_source_mismatch",
            Self::FamilySourceMismatch => "family_source_mismatch",
            Self::AdmissionSourceMismatch => "admission_source_mismatch",
            Self::AdmissionEvidenceRequired => "admission_evidence_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportReportError {
    denial_kind: QuerySubscriptionSupportReportDenialKind,
    message: &'static str,
    failure_digest: String,
}

impl QuerySubscriptionSupportReportError {
    fn new(
        denial_kind: QuerySubscriptionSupportReportDenialKind,
        message: &'static str,
        evidence_parts: &[String],
    ) -> Self {
        let mut parts = vec![
            "query_subscription_support_report_error_v1".to_string(),
            denial_kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence_parts.iter().cloned());
        Self {
            denial_kind,
            message,
            failure_digest: hash_parts(&parts),
        }
    }

    pub fn denial_kind(&self) -> &QuerySubscriptionSupportReportDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportReport {
    support_subject: QuerySubscriptionSupportSubject,
    support_posture: QuerySubscriptionSupportPosture,
    support_matrix: QuerySubscriptionSupportMatrix,
    source_digest: String,
    counter_snapshot: String,
    lookup_receipt_digest: String,
    report_digest: String,
    counters: QuerySubscriptionSupportCounters,
}

impl QuerySubscriptionSupportReport {
    pub fn support_subject(&self) -> &QuerySubscriptionSupportSubject {
        &self.support_subject
    }

    pub fn support_posture(&self) -> &QuerySubscriptionSupportPosture {
        &self.support_posture
    }

    pub fn support_matrix(&self) -> &QuerySubscriptionSupportMatrix {
        &self.support_matrix
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn lookup_receipt_digest(&self) -> &str {
        &self.lookup_receipt_digest
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionSupportCounters {
        &self.counters
    }
}

pub fn report_query_subscription_support(
    subject: QuerySubscriptionSupportSubject,
    evidence: QuerySubscriptionSupportEvidence,
) -> Result<
    (QuerySubscriptionSupportReport, SupportLookupReceipt),
    QuerySubscriptionSupportReportError,
> {
    validate_subject_matches_evidence(&subject, &evidence)?;

    let support_matrix = QuerySubscriptionSupportMatrix::for_family(
        evidence.family(),
        evidence.support_profile(),
        &subject,
    );
    let support_row = support_matrix
        .row_for_class(*subject.support_class())
        .expect("every support class must exist in the family support matrix");

    let counters = counters_for_posture(support_row.posture());
    let lookup_receipt = SupportLookupReceipt::new(
        evidence.family(),
        *subject.support_class(),
        SupportResolutionPosture::IndexedFamilyLookup,
        1,
        support_matrix.rows().len().saturating_sub(1),
    );
    let counter_snapshot = counters.digest();
    let report_digest = hash_parts(&[
        "query_subscription_support_report_v1".to_string(),
        format!("subject:{}", subject.digest()),
        format!("family:{}", evidence.family().as_str()),
        format!("posture:{}", support_row.posture().as_str()),
        format!("source:{}", subject.source_digest()),
        format!("support_matrix:{}", support_matrix.digest()),
        format!("lookup_receipt:{}", lookup_receipt.digest()),
        format!("counters:{counter_snapshot}"),
    ]);

    Ok((
        QuerySubscriptionSupportReport {
            support_subject: subject.clone(),
            support_posture: *support_row.posture(),
            support_matrix,
            source_digest: subject.source_digest().to_string(),
            counter_snapshot,
            lookup_receipt_digest: lookup_receipt.digest().to_string(),
            report_digest,
            counters,
        },
        lookup_receipt,
    ))
}

fn validate_subject_matches_evidence(
    subject: &QuerySubscriptionSupportSubject,
    evidence: &QuerySubscriptionSupportEvidence,
) -> Result<(), QuerySubscriptionSupportReportError> {
    if subject.declaration_digest() != evidence.declaration_digest() {
        return Err(QuerySubscriptionSupportReportError::new(
            QuerySubscriptionSupportReportDenialKind::DeclarationSourceMismatch,
            "subscription support reporting requires a subject built from the same declaration artifact",
            &[
                format!("subject_declaration:{}", subject.declaration_digest()),
                format!("evidence_declaration:{}", evidence.declaration_digest()),
            ],
        ));
    }

    if subject.family() != evidence.family() {
        return Err(QuerySubscriptionSupportReportError::new(
            QuerySubscriptionSupportReportDenialKind::FamilySourceMismatch,
            "subscription support reporting requires subject and evidence to preserve the same query subscription family",
            &[
                format!("subject_family:{}", subject.family().as_str()),
                format!("evidence_family:{}", evidence.family().as_str()),
            ],
        ));
    }

    match (subject.admission_digest(), evidence.admission_digest()) {
        (Some(subject_admission_digest), Some(evidence_admission_digest)) => {
            if subject_admission_digest != evidence_admission_digest {
                return Err(QuerySubscriptionSupportReportError::new(
                    QuerySubscriptionSupportReportDenialKind::AdmissionSourceMismatch,
                    "subscription support reporting requires a subject bound to the same admission artifact",
                    &[
                        format!("subject_admission:{subject_admission_digest}"),
                        format!("evidence_admission:{evidence_admission_digest}"),
                    ],
                ));
            }
        }
        (Some(subject_admission_digest), None) => {
            return Err(QuerySubscriptionSupportReportError::new(
                QuerySubscriptionSupportReportDenialKind::AdmissionEvidenceRequired,
                "subscription support reporting requires admission evidence for activation, lifecycle, continuation, and preview subjects",
                &[
                    format!("subject_support_class:{}", subject.support_class().as_str()),
                    format!("subject_admission:{subject_admission_digest}"),
                    format!("evidence_source:{}", evidence.source_digest()),
                ],
            ));
        }
        (None, Some(_)) if subject.support_class().requires_admission_evidence() => {
            return Err(QuerySubscriptionSupportReportError::new(
                QuerySubscriptionSupportReportDenialKind::AdmissionEvidenceRequired,
                "subscription support reporting requires admission-bound subjects for activation, lifecycle, continuation, and preview support",
                &[
                    format!("subject_support_class:{}", subject.support_class().as_str()),
                    format!("evidence_source:{}", evidence.source_digest()),
                ],
            ));
        }
        _ => {}
    }

    Ok(())
}
fn counters_for_posture(
    posture: &QuerySubscriptionSupportPosture,
) -> QuerySubscriptionSupportCounters {
    let mut counters = QuerySubscriptionSupportCounters {
        support_report_request_count: 1,
        support_matrix_emission_count: 1,
        support_family_index_lookup_count: 1,
        ..Default::default()
    };
    match posture {
        QuerySubscriptionSupportPosture::RuntimeBackedCertified => {
            counters.supported_family_count = 1;
        }
        QuerySubscriptionSupportPosture::RuntimeBackedDenied => {
            counters.denied_family_count = 1;
        }
        QuerySubscriptionSupportPosture::RuntimeBackedDeferred => {
            counters.deferred_family_count = 1;
        }
        QuerySubscriptionSupportPosture::UncertifiedDenied => {
            counters.uncertified_family_denial_count = 1;
            counters.denied_family_count = 1;
        }
    }
    counters
}
