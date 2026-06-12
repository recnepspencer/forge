use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeCloseoutPosture {
    SeamEliminated,
    DeferredNeighbor,
}

impl ForgeQueryLowerRuntimeCloseoutPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SeamEliminated => "seam-eliminated",
            Self::DeferredNeighbor => "deferred-neighbor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeCloseoutRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &'static str,
    posture: ForgeQueryLowerRuntimeCloseoutPosture,
    owner: ForgeQueryLowerRuntimeAuthorityOwner,
    route_kind: ForgeQueryLowerRuntimeRouteKind,
    closeout_target: &'static str,
    required_closeout: &'static str,
    certification_row: &'static str,
}

impl ForgeQueryLowerRuntimeCloseoutRow {
    pub(crate) const fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        capability_label: &'static str,
        posture: ForgeQueryLowerRuntimeCloseoutPosture,
        owner: ForgeQueryLowerRuntimeAuthorityOwner,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        closeout_target: &'static str,
        required_closeout: &'static str,
        certification_row: &'static str,
    ) -> Self {
        Self {
            seam_key,
            capability_label,
            posture,
            owner,
            route_kind,
            closeout_target,
            required_closeout,
            certification_row,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn capability_label(&self) -> &'static str {
        self.capability_label
    }

    pub fn posture(&self) -> ForgeQueryLowerRuntimeCloseoutPosture {
        self.posture
    }

    pub fn owner(&self) -> ForgeQueryLowerRuntimeAuthorityOwner {
        self.owner
    }

    pub fn route_kind(&self) -> ForgeQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn closeout_target(&self) -> &'static str {
        self.closeout_target
    }

    pub fn required_closeout(&self) -> &'static str {
        self.required_closeout
    }

    pub fn certification_row(&self) -> &'static str {
        self.certification_row
    }

    pub fn row_digest(&self) -> String {
        self.row_identity().as_str().to_string()
    }

    fn row_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_closeout_row_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("seam"), self.seam_key.as_str())
            .field_shape(
                ForgeQueryEvidenceTag::new("capability"),
                self.capability_label,
            )
            .field_shape(ForgeQueryEvidenceTag::new("posture"), self.posture.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("owner"), self.owner.as_str())
            .field_shape(
                ForgeQueryEvidenceTag::new("route_kind"),
                self.route_kind.as_str(),
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("closeout_target"),
                self.closeout_target,
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("required_closeout"),
                self.required_closeout,
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("certification_row"),
                self.certification_row,
            )
            .seal()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeCloseoutRegistry {
    rows: &'static [ForgeQueryLowerRuntimeCloseoutRow],
}

impl ForgeQueryLowerRuntimeCloseoutRegistry {
    pub(crate) const fn new(rows: &'static [ForgeQueryLowerRuntimeCloseoutRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryLowerRuntimeCloseoutRow] {
        self.rows
    }

    pub fn registry_digest(&self) -> String {
        let row_identities = self
            .rows
            .iter()
            .map(ForgeQueryLowerRuntimeCloseoutRow::row_identity)
            .collect::<Vec<_>>();
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_closeout_registry_v1",
            )
            .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
            .seal()
            .as_str()
            .to_string()
    }

    pub fn required_closeout_digest(&self) -> String {
        let closeout_identities = self
            .rows
            .iter()
            .map(|row| {
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_identity(
                    ForgeQueryEvidenceTag::new("required_closeout"),
                    row.required_closeout(),
                )
                .seal()
            })
            .collect::<Vec<_>>();
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_required_closeout_v1",
            )
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("rows"),
                &closeout_identities,
            )
            .seal()
            .as_str()
            .to_string()
    }
}
