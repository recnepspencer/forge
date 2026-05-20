use crate::identity::hash_parts;

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
        hash_parts(&[
            "lower_runtime_closeout_row_v1".to_string(),
            format!("seam:{}", self.seam_key.as_str()),
            format!("capability:{}", self.capability_label),
            format!("posture:{}", self.posture.as_str()),
            format!("owner:{}", self.owner.as_str()),
            format!("route_kind:{}", self.route_kind.as_str()),
            format!("closeout_target:{}", self.closeout_target),
            format!("required_closeout:{}", self.required_closeout),
            format!("certification_row:{}", self.certification_row),
        ])
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
        hash_parts(
            &self
                .rows
                .iter()
                .map(ForgeQueryLowerRuntimeCloseoutRow::row_digest)
                .collect::<Vec<_>>(),
        )
    }

    pub fn required_closeout_digest(&self) -> String {
        hash_parts(
            &self
                .rows
                .iter()
                .map(|row| row.required_closeout().to_string())
                .collect::<Vec<_>>(),
        )
    }
}
