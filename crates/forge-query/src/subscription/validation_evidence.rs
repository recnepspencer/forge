use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::terminal_projection_label::TerminalProjectionLabel;

pub(crate) fn validation_evidence_identity_label(
    identity: &ForgeQueryEvidenceIdentity,
) -> TerminalProjectionLabel {
    TerminalProjectionLabel::from_identity(identity)
}

pub(crate) fn validation_role_evidence_identity(
    role: &'static str,
    identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("subject"), identity)
        .seal()
}

pub(crate) fn validation_shape_role_evidence_identity(
    role: &'static str,
    shape: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(ForgeQueryEvidenceTag::new("subject"), shape)
        .seal()
}

pub(crate) fn validation_u64_role_evidence_identity(
    role: &'static str,
    value: u64,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_usize(ForgeQueryEvidenceTag::new("subject"), value as usize)
        .seal()
}

pub(crate) fn validation_usize_role_evidence_identity(
    role: &'static str,
    value: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_usize(ForgeQueryEvidenceTag::new("subject"), value)
        .seal()
}

pub(crate) fn validation_label_list_evidence_identity(
    role: &'static str,
    labels: &[String],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("subject"),
            labels.iter().map(String::as_str),
        )
        .seal()
}
