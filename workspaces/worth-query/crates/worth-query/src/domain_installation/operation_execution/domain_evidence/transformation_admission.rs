use worth_query_installation::facade::WorthQueryTransformationEvidenceContract;

use super::{
    WorthQueryDomainEvidenceAdmissionDenial, WorthQueryDomainEvidenceAdmissionDenialKind,
    WorthQueryTransformationRecord, WorthQueryTransformationSummary,
};

pub(super) fn admit_transformation(
    contract: &WorthQueryTransformationEvidenceContract,
    summary: Option<WorthQueryTransformationSummary>,
    records: Option<&[WorthQueryTransformationRecord]>,
    output_occurrence_identity: &str,
) -> Result<Option<WorthQueryTransformationSummary>, WorthQueryDomainEvidenceAdmissionDenial> {
    let WorthQueryTransformationEvidenceContract::Declared {
        source_occurrence,
        transformation,
        outcome,
    } = contract
    else {
        if summary.is_none() && records.is_none() {
            return Ok(None);
        }
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::UnexpectedTransformationSummary,
            "transformation-not-declared",
        ));
    };
    let summary = summary.ok_or_else(|| {
        denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::MissingTransformationSummary,
            transformation.family(),
        )
    })?;
    let parts = summary.parts();
    if parts.source_occurrence.family() != source_occurrence.identity_family()
        || !portable(parts.source_occurrence.value())
        || parts.output_occurrence_identity != output_occurrence_identity
        || parts.transformation_family != transformation.family()
        || parts.transformation_version != transformation.version()
        || parts.correspondence != outcome.correspondence()
        || parts.disposition != outcome.disposition()
        || parts.error != outcome.error()
        || parts.loss != outcome.loss()
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::TransformationSummaryMismatch,
            transformation.family(),
        ));
    }
    if let Some(records) = records {
        validate_records(&summary, records)?;
    }
    Ok(Some(summary))
}

fn validate_records(
    summary: &WorthQueryTransformationSummary,
    records: &[WorthQueryTransformationRecord],
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let parts = summary.parts();
    let valid = !records.is_empty()
        && records.iter().all(|record| {
            portable(record.source_occurrence_identity())
                && !record.output_occurrence_identities().is_empty()
                && record
                    .output_occurrence_identities()
                    .iter()
                    .all(|identity| portable(identity))
                && record.disposition() == parts.disposition
                && record.error() == parts.error
        })
        && records.iter().any(|record| {
            record
                .output_occurrence_identities()
                .iter()
                .any(|identity| identity == &parts.output_occurrence_identity)
        });
    valid.then_some(()).ok_or_else(|| {
        denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::TransformationSidecarMismatch,
            &parts.transformation_family,
        )
    })
}

fn denial(
    kind: WorthQueryDomainEvidenceAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryDomainEvidenceAdmissionDenial {
    WorthQueryDomainEvidenceAdmissionDenial::new(kind, subject)
}

fn portable(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
