use super::contracts::{
    HistoricalPathReuseDescriptor, HistoricalReconstructionBudget, HistoricalReplaySpanBudget,
};
use super::cost::HistoricalPathCostPosture;
use super::error::HistoricalEvaluationError;
use super::path_classes::{
    AdmittedHistoricalPathClass, RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalEvaluationRequest {
    basis_identity: String,
    requested_path_class: RequestedHistoricalPathClass,
    cost_posture: HistoricalPathCostPosture,
    replay_budget: HistoricalReplaySpanBudget,
    reconstruction_budget: HistoricalReconstructionBudget,
    reuse_descriptor: HistoricalPathReuseDescriptor,
}

impl HistoricalEvaluationRequest {
    pub fn retained_snapshot(
        basis_identity: impl Into<String>,
        replay_budget: usize,
        reconstruction_budget: usize,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self::new(
            basis_identity,
            RequestedHistoricalPathClass::RequestedRetainedSnapshotPath,
            HistoricalPathCostPosture::HistoricalRetainedFastPath,
            HistoricalReplaySpanBudget::bounded(replay_budget),
            HistoricalReconstructionBudget::bounded(reconstruction_budget),
            reuse_descriptor,
        )
    }

    pub fn delta_replay(
        basis_identity: impl Into<String>,
        replay_budget: usize,
        reconstruction_budget: usize,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self::new(
            basis_identity,
            RequestedHistoricalPathClass::RequestedDeltaReplayPath,
            HistoricalPathCostPosture::HistoricalReplayBounded,
            HistoricalReplaySpanBudget::bounded(replay_budget),
            HistoricalReconstructionBudget::bounded(reconstruction_budget),
            reuse_descriptor,
        )
    }

    pub fn full_reconstruction(
        basis_identity: impl Into<String>,
        replay_budget: usize,
        reconstruction_budget: usize,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self::new(
            basis_identity,
            RequestedHistoricalPathClass::RequestedFullReconstructionPath,
            HistoricalPathCostPosture::HistoricalReconstructionExpensive,
            HistoricalReplaySpanBudget::bounded(replay_budget),
            HistoricalReconstructionBudget::bounded(reconstruction_budget),
            reuse_descriptor,
        )
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn requested_path_class(&self) -> &RequestedHistoricalPathClass {
        &self.requested_path_class
    }

    pub fn cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.cost_posture
    }

    pub fn replay_budget(&self) -> &HistoricalReplaySpanBudget {
        &self.replay_budget
    }

    pub fn reconstruction_budget(&self) -> &HistoricalReconstructionBudget {
        &self.reconstruction_budget
    }

    pub fn reuse_descriptor(&self) -> &HistoricalPathReuseDescriptor {
        &self.reuse_descriptor
    }

    pub(crate) fn new(
        basis_identity: impl Into<String>,
        requested_path_class: RequestedHistoricalPathClass,
        cost_posture: HistoricalPathCostPosture,
        replay_budget: HistoricalReplaySpanBudget,
        reconstruction_budget: HistoricalReconstructionBudget,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            requested_path_class,
            cost_posture,
            replay_budget,
            reconstruction_budget,
            reuse_descriptor,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathRequested {
    basis_identity: String,
    requested_path_class: RequestedHistoricalPathClass,
}

impl HistoricalPathRequested {
    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn requested_path_class(&self) -> &RequestedHistoricalPathClass {
        &self.requested_path_class
    }

    pub(crate) fn from_request(request: &HistoricalEvaluationRequest) -> Self {
        Self {
            basis_identity: request.basis_identity.clone(),
            requested_path_class: request.requested_path_class.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalCapabilityDescriptor {
    basis_identity: String,
    admitted_path_class: Option<AdmittedHistoricalPathClass>,
    replay_permitted: bool,
    replay_required: bool,
    retention_available: bool,
    historical_lookup_available: bool,
    reuse_descriptor: HistoricalPathReuseDescriptor,
}

impl HistoricalCapabilityDescriptor {
    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn admitted_path_class(&self) -> Option<&AdmittedHistoricalPathClass> {
        self.admitted_path_class.as_ref()
    }

    pub fn reuse_descriptor(&self) -> &HistoricalPathReuseDescriptor {
        &self.reuse_descriptor
    }

    pub fn retained_snapshot(
        basis_identity: impl Into<String>,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            admitted_path_class: Some(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
            replay_permitted: false,
            replay_required: false,
            retention_available: true,
            historical_lookup_available: false,
            reuse_descriptor,
        }
    }

    pub fn delta_replay(
        basis_identity: impl Into<String>,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            admitted_path_class: Some(AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
            replay_permitted: true,
            replay_required: false,
            retention_available: false,
            historical_lookup_available: true,
            reuse_descriptor,
        }
    }

    pub fn full_reconstruction(
        basis_identity: impl Into<String>,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            admitted_path_class: Some(AdmittedHistoricalPathClass::AdmittedFullReconstructionPath),
            replay_permitted: true,
            replay_required: false,
            retention_available: false,
            historical_lookup_available: true,
            reuse_descriptor,
        }
    }

    pub(crate) fn replay_permitted(&self) -> bool {
        self.replay_permitted
    }

    pub(crate) fn replay_required(&self) -> bool {
        self.replay_required
    }

    pub(crate) fn retention_available(&self) -> bool {
        self.retention_available
    }

    pub(crate) fn historical_lookup_available(&self) -> bool {
        self.historical_lookup_available
    }

    pub(crate) fn new(
        basis_identity: impl Into<String>,
        admitted_path_class: Option<AdmittedHistoricalPathClass>,
        replay_permitted: bool,
        replay_required: bool,
        retention_available: bool,
        historical_lookup_available: bool,
        reuse_descriptor: HistoricalPathReuseDescriptor,
    ) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            admitted_path_class,
            replay_permitted,
            replay_required,
            retention_available,
            historical_lookup_available,
            reuse_descriptor,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalMaterializationDescriptor {
    basis_identity: String,
    resolved_path_class: ResolvedHistoricalPathClass,
    actual_replay_span: usize,
    actual_reconstruction_scope: usize,
}

impl HistoricalMaterializationDescriptor {
    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn resolved_path_class(&self) -> &ResolvedHistoricalPathClass {
        &self.resolved_path_class
    }

    pub fn actual_replay_span(&self) -> usize {
        self.actual_replay_span
    }

    pub fn actual_reconstruction_scope(&self) -> usize {
        self.actual_reconstruction_scope
    }

    pub fn retained_snapshot(basis_identity: impl Into<String>) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            resolved_path_class: ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
            actual_replay_span: 0,
            actual_reconstruction_scope: 0,
        }
    }

    pub fn delta_replay(basis_identity: impl Into<String>) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            resolved_path_class: ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
            actual_replay_span: 1,
            actual_reconstruction_scope: 0,
        }
    }

    pub fn full_reconstruction(basis_identity: impl Into<String>) -> Self {
        Self {
            basis_identity: basis_identity.into(),
            resolved_path_class: ResolvedHistoricalPathClass::ResolvedFullReconstructionPath,
            actual_replay_span: 0,
            actual_reconstruction_scope: 1,
        }
    }

    pub fn with_realized_work(
        mut self,
        actual_replay_span: usize,
        actual_reconstruction_scope: usize,
    ) -> Self {
        self.actual_replay_span = actual_replay_span;
        self.actual_reconstruction_scope = actual_reconstruction_scope;
        self
    }

    pub(crate) fn new(
        basis_identity: impl Into<String>,
        resolved_path_class: ResolvedHistoricalPathClass,
    ) -> Self {
        let (actual_replay_span, actual_reconstruction_scope) =
            default_realized_work(&resolved_path_class);
        Self {
            basis_identity: basis_identity.into(),
            resolved_path_class,
            actual_replay_span,
            actual_reconstruction_scope,
        }
    }
}

pub(crate) fn validate_basis_match(
    request: &HistoricalEvaluationRequest,
    descriptor_basis: &str,
) -> Result<(), HistoricalEvaluationError> {
    if request.basis_identity() == descriptor_basis {
        Ok(())
    } else {
        Err(HistoricalEvaluationError::IncompatibleBasisPathPair {
            requested_basis_identity: request.basis_identity().to_string(),
            descriptor_basis_identity: descriptor_basis.to_string(),
            requested_path_class: request.requested_path_class().clone(),
        })
    }
}

fn default_realized_work(resolved_path_class: &ResolvedHistoricalPathClass) -> (usize, usize) {
    match resolved_path_class {
        ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath => (0, 0),
        ResolvedHistoricalPathClass::ResolvedDeltaReplayPath => (1, 0),
        ResolvedHistoricalPathClass::ResolvedFullReconstructionPath => (0, 1),
    }
}
