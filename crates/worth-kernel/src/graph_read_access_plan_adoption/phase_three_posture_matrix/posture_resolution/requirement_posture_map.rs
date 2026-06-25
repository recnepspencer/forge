use std::collections::HashSet;

use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout;

use super::super::errors::{
    WorthGraphReadAccessPostureMatrixError, WorthGraphReadAccessPostureMatrixErrorKind,
};
use super::super::stable_digest;
use super::carried_gap_posture::resolve_carried_gap_posture;
use super::query_attempt_posture::resolve_query_attempt_posture;
use super::resolved_posture::WorthGraphReadAccessResolvedPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementPostureMap {
    resolved_postures: Vec<WorthGraphReadAccessResolvedPosture>,
    requirement_identity_count: usize,
    map_digest: String,
}

impl WorthGraphReadRequirementPostureMap {
    pub(crate) fn from_phase_two_closeout(
        phase_two: &WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout,
    ) -> Result<Self, WorthGraphReadAccessPostureMatrixError> {
        let mut resolved_postures = phase_two
            .posture_report()
            .posture_rows()
            .iter()
            .map(resolve_query_attempt_posture)
            .collect::<Vec<_>>();
        resolved_postures.extend(
            phase_two
                .adoption_ledger()
                .carried_capability_gaps()
                .iter()
                .map(resolve_carried_gap_posture),
        );
        Self::from_rows(resolved_postures)
    }

    pub(crate) fn from_rows(
        resolved_postures: Vec<WorthGraphReadAccessResolvedPosture>,
    ) -> Result<Self, WorthGraphReadAccessPostureMatrixError> {
        if resolved_postures.is_empty() {
            return Err(error(
                WorthGraphReadAccessPostureMatrixErrorKind::MissingResolvedPostureRows,
            ));
        }

        let mut identities = HashSet::new();
        for row in &resolved_postures {
            if !identities.insert(row.requirement_identity()) {
                return Err(error(
                    WorthGraphReadAccessPostureMatrixErrorKind::DuplicateResolvedRequirementPosture,
                ));
            }
        }

        let mut digest_parts = vec![
            "worth_graph_read_requirement_posture_map_v1".to_string(),
            format!("resolved_posture_count:{}", resolved_postures.len()),
            format!("requirement_identity_count:{}", identities.len()),
        ];
        digest_parts.extend(
            resolved_postures
                .iter()
                .map(|row| format!("resolved_posture:{}", row.row_digest())),
        );

        let requirement_identity_count = identities.len();
        Ok(Self {
            resolved_postures,
            requirement_identity_count,
            map_digest: stable_digest(&digest_parts),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_rows_for_tests(
        resolved_postures: Vec<WorthGraphReadAccessResolvedPosture>,
    ) -> Result<Self, WorthGraphReadAccessPostureMatrixError> {
        Self::from_rows(resolved_postures)
    }

    pub fn resolved_postures(&self) -> &[WorthGraphReadAccessResolvedPosture] {
        &self.resolved_postures
    }

    pub const fn requirement_identity_count(&self) -> usize {
        self.requirement_identity_count
    }

    pub fn map_digest(&self) -> &str {
        &self.map_digest
    }

    pub fn posture_for_requirement(
        &self,
        requirement_identity: &str,
    ) -> Option<&WorthGraphReadAccessResolvedPosture> {
        self.resolved_postures
            .iter()
            .find(|row| row.requirement_identity() == requirement_identity)
    }
}

const fn error(
    kind: WorthGraphReadAccessPostureMatrixErrorKind,
) -> WorthGraphReadAccessPostureMatrixError {
    WorthGraphReadAccessPostureMatrixError::new(kind)
}
