use super::policy::{
    PlanarBooleanEdgeSplitDegeneracyPolicy, PlanarBooleanEdgeSplitDeterminismPolicy,
    PlanarBooleanEdgeSplitOverlapPolicy,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest;

#[derive(Clone, Copy)]
pub struct PlanarBooleanEdgeSplitScopeAdmissionInput<'a> {
    split_request: &'a PlanarBooleanEdgeSplitRequest,
    degeneracy_policy: PlanarBooleanEdgeSplitDegeneracyPolicy,
    determinism_policy: PlanarBooleanEdgeSplitDeterminismPolicy,
    overlap_policy: PlanarBooleanEdgeSplitOverlapPolicy,
}

impl<'a> PlanarBooleanEdgeSplitScopeAdmissionInput<'a> {
    pub fn from_split_request(split_request: &'a PlanarBooleanEdgeSplitRequest) -> Self {
        Self {
            split_request,
            degeneracy_policy: PlanarBooleanEdgeSplitDegeneracyPolicy::default(),
            determinism_policy: PlanarBooleanEdgeSplitDeterminismPolicy::default(),
            overlap_policy: PlanarBooleanEdgeSplitOverlapPolicy::default(),
        }
    }

    pub fn with_degeneracy_policy(
        mut self,
        policy: PlanarBooleanEdgeSplitDegeneracyPolicy,
    ) -> Self {
        self.degeneracy_policy = policy;
        self
    }

    pub fn with_determinism_policy(
        mut self,
        policy: PlanarBooleanEdgeSplitDeterminismPolicy,
    ) -> Self {
        self.determinism_policy = policy;
        self
    }

    pub fn with_overlap_policy(mut self, policy: PlanarBooleanEdgeSplitOverlapPolicy) -> Self {
        self.overlap_policy = policy;
        self
    }

    pub(crate) fn split_request(&self) -> &'a PlanarBooleanEdgeSplitRequest {
        self.split_request
    }

    pub(crate) fn degeneracy_policy(&self) -> PlanarBooleanEdgeSplitDegeneracyPolicy {
        self.degeneracy_policy
    }

    pub(crate) fn determinism_policy(&self) -> PlanarBooleanEdgeSplitDeterminismPolicy {
        self.determinism_policy
    }

    pub(crate) fn overlap_policy(&self) -> PlanarBooleanEdgeSplitOverlapPolicy {
        self.overlap_policy
    }
}
