use crate::evidence_identity::forge_query_evidence_identity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::field::{EvidenceReportField, EvidenceReportFieldValue};
use super::scope::EvidenceReportScope;

pub(crate) struct EvidenceReportIdentities {
    pub(crate) report_identity: ForgeQueryEvidenceIdentity,
    pub(crate) field_inventory_identity: ForgeQueryEvidenceIdentity,
    pub(crate) digest_participation_identity: ForgeQueryEvidenceIdentity,
}

pub(crate) fn derive_evidence_report_identities(
    scope: &EvidenceReportScope,
    report_name: &str,
    fields: &[EvidenceReportField],
) -> EvidenceReportIdentities {
    let field_inventory_identities = derive_field_inventory_identities(fields);
    let participating_inventory_identities =
        select_participating_inventory_identities(fields, &field_inventory_identities);
    let field_inventory_identity = derive_report_field_inventory_identity(
        scope,
        report_name,
        fields.len(),
        &field_inventory_identities,
    );
    let digest_participation_identity = derive_report_digest_participation_identity(
        scope,
        report_name,
        &participating_inventory_identities,
    );
    let participating_value_identities = derive_participating_field_value_identities(fields);
    let report_identity = derive_report_value_identity(
        scope,
        report_name,
        &digest_participation_identity,
        &participating_value_identities,
    );

    EvidenceReportIdentities {
        report_identity,
        field_inventory_identity,
        digest_participation_identity,
    }
}

fn derive_field_inventory_identities(
    fields: &[EvidenceReportField],
) -> Vec<ForgeQueryEvidenceIdentity> {
    fields
        .iter()
        .map(derive_field_inventory_or_diagnostic_identity)
        .collect()
}

fn select_participating_inventory_identities<'a>(
    fields: &'a [EvidenceReportField],
    field_inventory_identities: &'a [ForgeQueryEvidenceIdentity],
) -> Vec<&'a ForgeQueryEvidenceIdentity> {
    fields
        .iter()
        .zip(field_inventory_identities.iter())
        .filter_map(|(field, identity)| {
            field
                .participation()
                .participates_in_report_identity()
                .then_some(identity)
        })
        .collect()
}

fn derive_report_field_inventory_identity(
    scope: &EvidenceReportScope,
    report_name: &str,
    field_count: usize,
    field_inventory_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReportFieldInventory)
        .field_shape(ForgeQueryEvidenceTag::new("report_scope"), scope.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("report_name"), report_name)
        .field_usize(ForgeQueryEvidenceTag::new("field_count"), field_count)
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("field_identity"),
            field_inventory_identities.iter(),
        )
        .seal()
}

fn derive_report_digest_participation_identity(
    scope: &EvidenceReportScope,
    report_name: &str,
    participating_inventory_identities: &[&ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(
        ForgeQueryEvidenceScope::ConsumerEvidenceReportDigestParticipation,
    )
    .field_shape(ForgeQueryEvidenceTag::new("report_scope"), scope.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("report_name"), report_name)
    .field_usize(
        ForgeQueryEvidenceTag::new("participating_field_count"),
        participating_inventory_identities.len(),
    )
    .field_evidence_identity_sequence(
        ForgeQueryEvidenceTag::new("participating_field_identity"),
        participating_inventory_identities.iter().copied(),
    )
    .seal()
}

fn derive_participating_field_value_identities(
    fields: &[EvidenceReportField],
) -> Vec<ForgeQueryEvidenceIdentity> {
    fields
        .iter()
        .filter(|field| field.participation().participates_in_report_identity())
        .map(derive_field_value_identity)
        .collect()
}

fn derive_report_value_identity(
    scope: &EvidenceReportScope,
    report_name: &str,
    digest_participation_identity: &ForgeQueryEvidenceIdentity,
    participating_value_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReport)
        .field_shape(ForgeQueryEvidenceTag::new("report_scope"), scope.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("report_name"), report_name)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("digest_participation_identity"),
            digest_participation_identity,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("participating_field_identity"),
            participating_value_identities.iter(),
        )
        .seal()
}

fn derive_field_inventory_identity(field: &EvidenceReportField) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReportField)
        .field_shape(ForgeQueryEvidenceTag::new("field_name"), field.name())
        .field_shape(
            ForgeQueryEvidenceTag::new("field_kind"),
            field.kind().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("participation"),
            field.participation().as_str(),
        )
        .seal()
}

fn derive_field_inventory_or_diagnostic_identity(
    field: &EvidenceReportField,
) -> ForgeQueryEvidenceIdentity {
    if field.participation().participates_in_report_identity() {
        derive_field_inventory_identity(field)
    } else {
        derive_field_value_identity(field)
    }
}

fn derive_field_value_identity(field: &EvidenceReportField) -> ForgeQueryEvidenceIdentity {
    let encoder =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReportField)
            .field_shape(ForgeQueryEvidenceTag::new("field_name"), field.name())
            .field_shape(
                ForgeQueryEvidenceTag::new("field_kind"),
                field.kind().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("participation"),
                field.participation().as_str(),
            );

    match field.value() {
        EvidenceReportFieldValue::Shape(value) => encoder
            .field_shape(ForgeQueryEvidenceTag::new("field_value"), value)
            .seal(),
        EvidenceReportFieldValue::Value(value) => encoder
            .field_value(ForgeQueryEvidenceTag::new("field_value"), value)
            .seal(),
        EvidenceReportFieldValue::Bool(value) => encoder
            .field_bool(ForgeQueryEvidenceTag::new("field_value"), *value)
            .seal(),
        EvidenceReportFieldValue::Usize(value) => encoder
            .field_usize(ForgeQueryEvidenceTag::new("field_value"), *value)
            .seal(),
        EvidenceReportFieldValue::EvidenceIdentity(value) => encoder
            .field_evidence_identity(ForgeQueryEvidenceTag::new("field_value"), value)
            .seal(),
        EvidenceReportFieldValue::ValueSequence(values) => encoder
            .field_value_sequence(ForgeQueryEvidenceTag::new("field_value"), values)
            .seal(),
        EvidenceReportFieldValue::EvidenceIdentitySequence(values) => encoder
            .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("field_value"), values)
            .seal(),
        EvidenceReportFieldValue::OptionalValue(value) => encoder
            .field_bool(ForgeQueryEvidenceTag::new("field_present"), value.is_some())
            .optional_value(ForgeQueryEvidenceTag::new("field_value"), value.as_deref())
            .seal(),
        EvidenceReportFieldValue::OptionalEvidenceIdentity(value) => encoder
            .field_bool(ForgeQueryEvidenceTag::new("field_present"), value.is_some())
            .optional_evidence_identity(ForgeQueryEvidenceTag::new("field_value"), value.as_ref())
            .seal(),
    }
}
