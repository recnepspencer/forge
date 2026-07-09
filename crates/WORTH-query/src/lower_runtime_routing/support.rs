use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::{
    worth_query_lower_runtime_closeout_registry, worth_query_lower_runtime_crossing_inventory,
    WorthQueryLowerRuntimeArtifactStrength, WorthQueryLowerRuntimeAuthorityOwner,
    WorthQueryLowerRuntimeCloseoutPosture, WorthQueryLowerRuntimeCrossingClassification,
    WorthQueryLowerRuntimeRouteKind, WorthQueryLowerRuntimeSeamKey,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeSupportPosture {
    Admitted,
    CompatibilityDebt,
    SeamEliminated,
    Deferred,
    Forbidden,
}

impl WorthQueryLowerRuntimeSupportPosture {
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
pub enum WorthQueryLowerRuntimeSupportDetail {
    Crossing,
    Closeout {
        closeout_target: &'static str,
        required_closeout: &'static str,
        certification_row: &'static str,
    },
}

impl WorthQueryLowerRuntimeSupportDetail {
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
pub struct WorthQueryLowerRuntimeSupportRow {
    seam_key: WorthQueryLowerRuntimeSeamKey,
    capability_label: &'static str,
    authority_owner: WorthQueryLowerRuntimeAuthorityOwner,
    route_kind: WorthQueryLowerRuntimeRouteKind,
    artifact_strength: WorthQueryLowerRuntimeArtifactStrength,
    posture: WorthQueryLowerRuntimeSupportPosture,
    detail: WorthQueryLowerRuntimeSupportDetail,
}

impl WorthQueryLowerRuntimeSupportRow {
    pub(crate) const fn new(
        seam_key: WorthQueryLowerRuntimeSeamKey,
        capability_label: &'static str,
        authority_owner: WorthQueryLowerRuntimeAuthorityOwner,
        route_kind: WorthQueryLowerRuntimeRouteKind,
        artifact_strength: WorthQueryLowerRuntimeArtifactStrength,
        posture: WorthQueryLowerRuntimeSupportPosture,
        detail: WorthQueryLowerRuntimeSupportDetail,
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

    pub fn seam_key(&self) -> WorthQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn capability_label(&self) -> &'static str {
        self.capability_label
    }

    pub fn authority_owner(&self) -> WorthQueryLowerRuntimeAuthorityOwner {
        self.authority_owner
    }

    pub fn route_kind(&self) -> WorthQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn artifact_strength(&self) -> WorthQueryLowerRuntimeArtifactStrength {
        self.artifact_strength
    }

    pub fn posture(&self) -> WorthQueryLowerRuntimeSupportPosture {
        self.posture
    }

    pub fn detail(&self) -> WorthQueryLowerRuntimeSupportDetail {
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
        let mut identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "lower_runtime_support_row_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("seam"), self.seam_key.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("capability"),
            self.capability_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("owner"),
            self.authority_owner.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("route_kind"),
            self.route_kind.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("artifact"),
            self.artifact_strength.as_str(),
        )
        .field_shape(WorthQueryEvidenceTag::new("posture"), self.posture.as_str());
        match self.detail {
            WorthQueryLowerRuntimeSupportDetail::Crossing => {
                identity = identity.field_shape(WorthQueryEvidenceTag::new("detail"), "crossing");
            }
            WorthQueryLowerRuntimeSupportDetail::Closeout {
                closeout_target,
                required_closeout,
                certification_row,
            } => {
                identity = identity
                    .field_shape(WorthQueryEvidenceTag::new("detail"), "closeout")
                    .field_value(
                        WorthQueryEvidenceTag::new("closeout_target"),
                        closeout_target,
                    )
                    .field_value(
                        WorthQueryEvidenceTag::new("required_closeout"),
                        required_closeout,
                    )
                    .field_value(
                        WorthQueryEvidenceTag::new("certification_row"),
                        certification_row,
                    );
            }
        }
        identity.seal().as_str().to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeSupportMatrix {
    rows: Vec<WorthQueryLowerRuntimeSupportRow>,
}

impl WorthQueryLowerRuntimeSupportMatrix {
    pub(crate) fn new(rows: Vec<WorthQueryLowerRuntimeSupportRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[WorthQueryLowerRuntimeSupportRow] {
        &self.rows
    }

    pub fn support_for(
        &self,
        seam_key: WorthQueryLowerRuntimeSeamKey,
    ) -> Option<&WorthQueryLowerRuntimeSupportRow> {
        self.rows.iter().find(|row| row.seam_key == seam_key)
    }

    pub fn matrix_digest(&self) -> String {
        let row_identities = self
            .rows
            .iter()
            .map(|row| {
                WorthQueryEvidenceIdentity::compose(
                    WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(WorthQueryEvidenceTag::new("support_row"), row.row_digest())
                .seal()
            })
            .collect::<Vec<_>>();
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "lower_runtime_support_matrix_v1",
            )
            .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("rows"), &row_identities)
            .seal()
            .as_str()
            .to_string()
    }
}

pub fn worth_query_lower_runtime_support_matrix() -> WorthQueryLowerRuntimeSupportMatrix {
    let mut rows: Vec<_> = worth_query_lower_runtime_crossing_inventory()
        .rows()
        .iter()
        .map(|row| {
            WorthQueryLowerRuntimeSupportRow::new(
                row.seam_key(),
                row.capability_label(),
                row.lower_runtime_owner(),
                row.route_kind(),
                row.current_artifact_strength(),
                support_posture_for_classification(row.classification()),
                WorthQueryLowerRuntimeSupportDetail::Crossing,
            )
        })
        .collect();
    rows.extend(
        worth_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .map(|row| {
                WorthQueryLowerRuntimeSupportRow::new(
                    row.seam_key(),
                    row.capability_label(),
                    row.owner(),
                    row.route_kind(),
                    WorthQueryLowerRuntimeArtifactStrength::DerivedAggregateArtifact,
                    support_posture_for_closeout(row.posture()),
                    WorthQueryLowerRuntimeSupportDetail::Closeout {
                        closeout_target: row.closeout_target(),
                        required_closeout: row.required_closeout(),
                        certification_row: row.certification_row(),
                    },
                )
            }),
    );
    WorthQueryLowerRuntimeSupportMatrix::new(rows)
}

pub(crate) fn support_posture_for_classification(
    classification: WorthQueryLowerRuntimeCrossingClassification,
) -> WorthQueryLowerRuntimeSupportPosture {
    match classification {
        WorthQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse
        | WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter => {
            WorthQueryLowerRuntimeSupportPosture::Admitted
        }
        WorthQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane => {
            WorthQueryLowerRuntimeSupportPosture::CompatibilityDebt
        }
        WorthQueryLowerRuntimeCrossingClassification::DeferredNeighbor => {
            WorthQueryLowerRuntimeSupportPosture::Deferred
        }
        WorthQueryLowerRuntimeCrossingClassification::ForbiddenDuplicate => {
            WorthQueryLowerRuntimeSupportPosture::Forbidden
        }
    }
}

pub(crate) fn support_posture_for_closeout(
    posture: WorthQueryLowerRuntimeCloseoutPosture,
) -> WorthQueryLowerRuntimeSupportPosture {
    match posture {
        WorthQueryLowerRuntimeCloseoutPosture::SeamEliminated => {
            WorthQueryLowerRuntimeSupportPosture::SeamEliminated
        }
        WorthQueryLowerRuntimeCloseoutPosture::DeferredNeighbor => {
            WorthQueryLowerRuntimeSupportPosture::Deferred
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        worth_query_lower_runtime_closeout_registry, WorthQueryLowerRuntimeCrossingClassification,
        WorthQueryLowerRuntimeSeamKey,
    };

    #[test]
    fn support_matrix_rows_cover_crossings_and_closeout_registry() {
        let inventory = worth_query_lower_runtime_crossing_inventory();
        let closeout = worth_query_lower_runtime_closeout_registry();
        let support = worth_query_lower_runtime_support_matrix();

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
                WorthQueryLowerRuntimeSupportDetail::Crossing
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
        let support = worth_query_lower_runtime_support_matrix();
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
        let support = worth_query_lower_runtime_support_matrix();

        let eliminated = support
            .support_for(WorthQueryLowerRuntimeSeamKey::RuntimeIntentModule)
            .expect("eliminated seam should still be explainable through support");
        assert_eq!(
            eliminated.posture(),
            WorthQueryLowerRuntimeSupportPosture::SeamEliminated
        );

        let deferred = support
            .support_for(WorthQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor)
            .expect("deferred neighbor should be explainable through the same support surface");
        assert_eq!(
            deferred.posture(),
            WorthQueryLowerRuntimeSupportPosture::Deferred
        );
    }

    #[test]
    fn adapter_and_reuse_rows_remain_admitted_support() {
        for classification in [
            WorthQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
            WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter,
        ] {
            assert_eq!(
                support_posture_for_classification(classification),
                WorthQueryLowerRuntimeSupportPosture::Admitted
            );
        }
    }
}
