use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    ForgeQueryLowerRuntimeArtifactStrength, ForgeQueryLowerRuntimeAuthorityOwner,
    ForgeQueryLowerRuntimeCloseoutPosture, ForgeQueryLowerRuntimeCrossingClassification,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeSupportPosture {
    Admitted,
    CompatibilityDebt,
    SeamEliminated,
    Deferred,
    Forbidden,
}

impl ForgeQueryLowerRuntimeSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::CompatibilityDebt => "compatibility-debt",
            Self::SeamEliminated => "seam-eliminated",
            Self::Deferred => "deferred",
            Self::Forbidden => "forbidden",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeSupportDetail {
    Crossing,
    Closeout {
        closeout_target: &'static str,
        required_closeout: &'static str,
        certification_row: &'static str,
    },
}

impl ForgeQueryLowerRuntimeSupportDetail {
    pub fn closeout_target(&self) -> Option<&'static str> {
        match self {
            Self::Crossing => None,
            Self::Closeout {
                closeout_target, ..
            } => Some(*closeout_target),
        }
    }

    pub fn required_closeout(&self) -> Option<&'static str> {
        match self {
            Self::Crossing => None,
            Self::Closeout {
                required_closeout, ..
            } => Some(*required_closeout),
        }
    }

    pub fn certification_row(&self) -> Option<&'static str> {
        match self {
            Self::Crossing => None,
            Self::Closeout {
                certification_row, ..
            } => Some(*certification_row),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSupportRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &'static str,
    authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
    route_kind: ForgeQueryLowerRuntimeRouteKind,
    artifact_strength: ForgeQueryLowerRuntimeArtifactStrength,
    posture: ForgeQueryLowerRuntimeSupportPosture,
    detail: ForgeQueryLowerRuntimeSupportDetail,
}

impl ForgeQueryLowerRuntimeSupportRow {
    pub(crate) const fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        capability_label: &'static str,
        authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        artifact_strength: ForgeQueryLowerRuntimeArtifactStrength,
        posture: ForgeQueryLowerRuntimeSupportPosture,
        detail: ForgeQueryLowerRuntimeSupportDetail,
    ) -> Self {
        Self {
            seam_key,
            capability_label,
            authority_owner,
            route_kind,
            artifact_strength,
            posture,
            detail,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn capability_label(&self) -> &'static str {
        self.capability_label
    }

    pub fn authority_owner(&self) -> ForgeQueryLowerRuntimeAuthorityOwner {
        self.authority_owner
    }

    pub fn route_kind(&self) -> ForgeQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn artifact_strength(&self) -> ForgeQueryLowerRuntimeArtifactStrength {
        self.artifact_strength
    }

    pub fn posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.posture
    }

    pub fn detail(&self) -> ForgeQueryLowerRuntimeSupportDetail {
        self.detail
    }

    pub fn closeout_target(&self) -> Option<&'static str> {
        self.detail.closeout_target()
    }

    pub fn required_closeout(&self) -> Option<&'static str> {
        self.detail.required_closeout()
    }

    pub fn certification_row(&self) -> Option<&'static str> {
        self.detail.certification_row()
    }

    pub fn row_digest(&self) -> String {
        let mut identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "lower_runtime_support_row_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("seam"), self.seam_key.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("capability"),
            self.capability_label,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("owner"),
            self.authority_owner.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("route_kind"),
            self.route_kind.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact"),
            self.artifact_strength.as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("posture"), self.posture.as_str());
        match self.detail {
            ForgeQueryLowerRuntimeSupportDetail::Crossing => {
                identity = identity.field_shape(ForgeQueryEvidenceTag::new("detail"), "crossing");
            }
            ForgeQueryLowerRuntimeSupportDetail::Closeout {
                closeout_target,
                required_closeout,
                certification_row,
            } => {
                identity = identity
                    .field_shape(ForgeQueryEvidenceTag::new("detail"), "closeout")
                    .field_value(
                        ForgeQueryEvidenceTag::new("closeout_target"),
                        closeout_target,
                    )
                    .field_value(
                        ForgeQueryEvidenceTag::new("required_closeout"),
                        required_closeout,
                    )
                    .field_value(
                        ForgeQueryEvidenceTag::new("certification_row"),
                        certification_row,
                    );
            }
        }
        identity.seal().as_str().to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSupportMatrix {
    rows: Vec<ForgeQueryLowerRuntimeSupportRow>,
}

impl ForgeQueryLowerRuntimeSupportMatrix {
    pub(crate) fn new(rows: Vec<ForgeQueryLowerRuntimeSupportRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimeSupportRow] {
        &self.rows
    }

    pub fn support_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeSupportRow> {
        self.rows.iter().find(|row| row.seam_key == seam_key)
    }

    pub fn matrix_digest(&self) -> String {
        let row_identities = self
            .rows
            .iter()
            .map(|row| {
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(ForgeQueryEvidenceTag::new("support_row"), row.row_digest())
                .seal()
            })
            .collect::<Vec<_>>();
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_support_matrix_v1",
            )
            .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
            .seal()
            .as_str()
            .to_string()
    }
}

pub fn forge_query_lower_runtime_support_matrix() -> ForgeQueryLowerRuntimeSupportMatrix {
    let mut rows: Vec<_> = forge_query_lower_runtime_crossing_inventory()
        .rows()
        .iter()
        .map(|row| {
            ForgeQueryLowerRuntimeSupportRow::new(
                row.seam_key(),
                row.capability_label(),
                row.lower_runtime_owner(),
                row.route_kind(),
                row.current_artifact_strength(),
                support_posture_for_classification(row.classification()),
                ForgeQueryLowerRuntimeSupportDetail::Crossing,
            )
        })
        .collect();
    rows.extend(
        forge_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .map(|row| {
                ForgeQueryLowerRuntimeSupportRow::new(
                    row.seam_key(),
                    row.capability_label(),
                    row.owner(),
                    row.route_kind(),
                    ForgeQueryLowerRuntimeArtifactStrength::DerivedAggregateArtifact,
                    support_posture_for_closeout(row.posture()),
                    ForgeQueryLowerRuntimeSupportDetail::Closeout {
                        closeout_target: row.closeout_target(),
                        required_closeout: row.required_closeout(),
                        certification_row: row.certification_row(),
                    },
                )
            }),
    );
    ForgeQueryLowerRuntimeSupportMatrix::new(rows)
}

pub(crate) fn support_posture_for_classification(
    classification: ForgeQueryLowerRuntimeCrossingClassification,
) -> ForgeQueryLowerRuntimeSupportPosture {
    match classification {
        ForgeQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse
        | ForgeQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter => {
            ForgeQueryLowerRuntimeSupportPosture::Admitted
        }
        ForgeQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane => {
            ForgeQueryLowerRuntimeSupportPosture::CompatibilityDebt
        }
        ForgeQueryLowerRuntimeCrossingClassification::DeferredNeighbor => {
            ForgeQueryLowerRuntimeSupportPosture::Deferred
        }
        ForgeQueryLowerRuntimeCrossingClassification::ForbiddenDuplicate => {
            ForgeQueryLowerRuntimeSupportPosture::Forbidden
        }
    }
}

pub(crate) fn support_posture_for_closeout(
    posture: ForgeQueryLowerRuntimeCloseoutPosture,
) -> ForgeQueryLowerRuntimeSupportPosture {
    match posture {
        ForgeQueryLowerRuntimeCloseoutPosture::SeamEliminated => {
            ForgeQueryLowerRuntimeSupportPosture::SeamEliminated
        }
        ForgeQueryLowerRuntimeCloseoutPosture::DeferredNeighbor => {
            ForgeQueryLowerRuntimeSupportPosture::Deferred
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        forge_query_lower_runtime_closeout_registry, ForgeQueryLowerRuntimeCrossingClassification,
        ForgeQueryLowerRuntimeSeamKey,
    };

    #[test]
    fn support_matrix_rows_cover_crossings_and_closeout_registry() {
        let inventory = forge_query_lower_runtime_crossing_inventory();
        let closeout = forge_query_lower_runtime_closeout_registry();
        let support = forge_query_lower_runtime_support_matrix();

        assert_eq!(
            support.rows().len(),
            inventory.rows().len() + closeout.rows().len()
        );
        for crossing in inventory.rows() {
            let support_row = support
                .support_for(crossing.seam_key())
                .expect("support matrix must cover every crossing row");
            assert_eq!(support_row.capability_label(), crossing.capability_label());
            assert_eq!(
                support_row.authority_owner(),
                crossing.lower_runtime_owner()
            );
            assert_eq!(support_row.route_kind(), crossing.route_kind());
            assert_eq!(
                support_row.posture(),
                support_posture_for_classification(crossing.classification())
            );
            assert_eq!(
                support_row.detail(),
                ForgeQueryLowerRuntimeSupportDetail::Crossing
            );
        }
        for row in closeout.rows() {
            let support_row = support
                .support_for(row.seam_key())
                .expect("support matrix must cover every closeout row");
            assert_eq!(support_row.capability_label(), row.capability_label());
            assert_eq!(support_row.authority_owner(), row.owner());
            assert_eq!(support_row.route_kind(), row.route_kind());
            assert_eq!(
                support_row.posture(),
                support_posture_for_closeout(row.posture())
            );
            assert_eq!(support_row.closeout_target(), Some(row.closeout_target()));
            assert_eq!(
                support_row.required_closeout(),
                Some(row.required_closeout())
            );
            assert_eq!(
                support_row.certification_row(),
                Some(row.certification_row())
            );
        }
    }

    #[test]
    fn support_matrix_rejects_seam_key_collisions_between_crossings_and_closeout_rows() {
        let support = forge_query_lower_runtime_support_matrix();
        let mut seen = std::collections::BTreeSet::new();

        for row in support.rows() {
            assert!(
                seen.insert(row.seam_key().as_str().to_string()),
                "support row seam key `{}` must be unique across crossing and closeout rows",
                row.seam_key().as_str()
            );
        }
    }

    #[test]
    fn seam_elimination_and_deferred_neighbors_share_one_support_lookup_surface() {
        let support = forge_query_lower_runtime_support_matrix();

        let eliminated = support
            .support_for(ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule)
            .expect("eliminated seam should still be explainable through support");
        assert_eq!(
            eliminated.posture(),
            ForgeQueryLowerRuntimeSupportPosture::SeamEliminated
        );

        let deferred = support
            .support_for(ForgeQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor)
            .expect("deferred neighbor should be explainable through the same support surface");
        assert_eq!(
            deferred.posture(),
            ForgeQueryLowerRuntimeSupportPosture::Deferred
        );
    }

    #[test]
    fn adapter_and_reuse_rows_remain_admitted_support() {
        for classification in [
            ForgeQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
            ForgeQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter,
        ] {
            assert_eq!(
                support_posture_for_classification(classification),
                ForgeQueryLowerRuntimeSupportPosture::Admitted
            );
        }
    }
}
