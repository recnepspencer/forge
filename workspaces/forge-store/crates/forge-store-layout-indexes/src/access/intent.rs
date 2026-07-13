use super::shape::{
    AccessAuthorityPosture, AccessLaneClassification, AccessShape, AccessShapeContract,
    AccessShapeDetail, AccessStaleDisposition, ExpectedCounterClass,
};
use crate::maintenance::PhysicalMutationShape;
use crate::materialization::AdmittedLayoutMaterialization;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedAccessIntent {
    detail: AccessShapeDetail,
    lane: AccessLaneClassification,
    authority_posture: AccessAuthorityPosture,
    stale_disposition: AccessStaleDisposition,
    expected_counters: ExpectedCounterClass,
    mutation_shape: Option<PhysicalMutationShape>,
    budget_rows: Option<u64>,
}

impl AdmittedAccessIntent {
    pub(crate) fn admit(
        declaration: AccessShapeContract,
        materialization: Option<&AdmittedLayoutMaterialization>,
    ) -> Option<Self> {
        let materialization_is_exact = materialization
            .is_some_and(|materialization| materialization.coverage().require_exact().is_ok());
        let materialization_matches_posture = match declaration.authority_posture() {
            AccessAuthorityPosture::ExactMaterialized
            | AccessAuthorityPosture::ExplicitDegradedExactScan => materialization_is_exact,
            AccessAuthorityPosture::MaintenanceMutation => materialization.is_none(),
        };
        if !materialization_matches_posture {
            return None;
        }

        Some(Self {
            detail: declaration.detail(),
            lane: declaration.lane(),
            authority_posture: declaration.authority_posture(),
            stale_disposition: declaration.stale_disposition(),
            expected_counters: declaration.expected_counters(),
            mutation_shape: declaration.mutation_shape(),
            budget_rows: declaration.budget_rows(),
        })
    }

    pub const fn shape(self) -> AccessShape {
        self.detail.shape()
    }

    pub const fn detail(self) -> AccessShapeDetail {
        self.detail
    }

    pub const fn lane(self) -> AccessLaneClassification {
        self.lane
    }

    pub const fn authority_posture(self) -> AccessAuthorityPosture {
        self.authority_posture
    }

    pub const fn stale_disposition(self) -> AccessStaleDisposition {
        self.stale_disposition
    }

    pub const fn expected_counters(self) -> ExpectedCounterClass {
        self.expected_counters
    }

    pub const fn mutation_shape(self) -> Option<PhysicalMutationShape> {
        self.mutation_shape
    }

    pub const fn budget_rows(self) -> Option<u64> {
        self.budget_rows
    }
}
