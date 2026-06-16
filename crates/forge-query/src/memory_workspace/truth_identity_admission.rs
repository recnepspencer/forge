use std::sync::Arc;

use forge_foundational::facade::admit_foundational_external_identity_token;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::identity_authority::{
    query_truth_identity_admission_authority, QueryCommitIdentityKind, QueryEntityIdentityKind,
    QueryExternalIdentityToken, QuerySnapshotIdentityKind,
    QueryTruthIdentityAdmissionAuthorityIdentity,
};

use super::{ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity};

fn preview_commit_evidence_from_admitted(
    admitted: &QueryTruthIdentityAdmissionAuthorityIdentity<Arc<str>, QueryCommitIdentityKind>,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WriteReceiptCommitIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_admitted_external_commit_label_v1",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("external_commit_label"),
            admitted.value().as_ref(),
        )
        .seal()
}

fn preview_snapshot_evidence_from_admitted(
    admitted: &QueryTruthIdentityAdmissionAuthorityIdentity<Arc<str>, QuerySnapshotIdentityKind>,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_admitted_external_snapshot_label_v1",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("external_snapshot_label"),
            admitted.value().as_ref(),
        )
        .seal()
}

fn preview_entity_evidence_from_admitted(
    admitted: &QueryTruthIdentityAdmissionAuthorityIdentity<Arc<str>, QueryEntityIdentityKind>,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::AuthoredCommandEntityIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_admitted_authored_entity_command_v1",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("authored_entity_identity"),
            admitted.value().as_ref(),
        )
        .seal()
}

pub fn admit_external_commit_token(
    token: QueryExternalIdentityToken<Arc<str>, QueryCommitIdentityKind>,
) -> ForgeQueryCommitIdentity {
    let admitted = admit_foundational_external_identity_token(
        token,
        query_truth_identity_admission_authority(),
    );
    ForgeQueryCommitIdentity::preview(preview_commit_evidence_from_admitted(&admitted))
}

pub fn admit_external_snapshot_token(
    token: QueryExternalIdentityToken<Arc<str>, QuerySnapshotIdentityKind>,
) -> ForgeQuerySnapshotIdentity {
    let admitted = admit_foundational_external_identity_token(
        token,
        query_truth_identity_admission_authority(),
    );
    ForgeQuerySnapshotIdentity::preview(preview_snapshot_evidence_from_admitted(&admitted))
}

pub fn admit_authored_entity_token(
    token: QueryExternalIdentityToken<Arc<str>, QueryEntityIdentityKind>,
) -> ForgeQueryEntityIdentity {
    let admitted = admit_foundational_external_identity_token(
        token,
        query_truth_identity_admission_authority(),
    );
    ForgeQueryEntityIdentity::preview(preview_entity_evidence_from_admitted(&admitted))
}

pub(crate) fn admit_external_commit_label(label: impl AsRef<str>) -> ForgeQueryCommitIdentity {
    admit_external_commit_token(QueryExternalIdentityToken::new(Arc::from(label.as_ref())))
}

pub(crate) fn admit_external_snapshot_label(label: impl AsRef<str>) -> ForgeQuerySnapshotIdentity {
    admit_external_snapshot_token(QueryExternalIdentityToken::new(Arc::from(label.as_ref())))
}

pub(crate) fn admit_authored_entity_label(label: impl AsRef<str>) -> ForgeQueryEntityIdentity {
    admit_authored_entity_token(QueryExternalIdentityToken::new(Arc::from(label.as_ref())))
}
