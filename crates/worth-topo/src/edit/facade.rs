use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::{
    RawWorthTopologyIntent, VerifiedTopologyCommit, WorthBoundaryEnvelope, WorthBoundaryFailure,
    WorthDecisionTrace, WorthIntegrityMarkers, WorthPerformanceAccounting, WorthTopologyAuthority,
    WorthTopologyAuthorityError, WorthTracedTopologyCommit,
};

use crate::reader::{WorthTopologyReadError, WorthTopologyReader};
use crate::validators::{
    validate_named_topology_truth, DerivedTopologyValidationReport, WorthTopologyValidationError,
};

use super::types::{WorthTopologyEditContract, WorthTopologyEditNamingReport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthTopologyEditApplicationMode {
    Mainline,
    BranchLocal(BranchId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyEditBatch {
    contracts: Vec<WorthTopologyEditContract>,
}

impl WorthTopologyEditBatch {
    pub fn new(contracts: Vec<WorthTopologyEditContract>) -> Result<Self, WorthTopologyEditError> {
        if contracts.is_empty() {
            return Err(WorthTopologyEditError::EmptyBatch);
        }
        Ok(Self { contracts })
    }

    pub fn contracts(&self) -> &[WorthTopologyEditContract] {
        &self.contracts
    }

    pub fn naming_report(&self) -> WorthTopologyEditNamingReport {
        let rows = self
            .contracts
            .iter()
            .flat_map(|contract| contract.naming_report().rows)
            .collect();
        WorthTopologyEditNamingReport { rows }
    }

    pub fn families(&self) -> Vec<super::types::WorthTopologyEditFamily> {
        self.contracts
            .iter()
            .map(|contract| contract.family)
            .collect()
    }

    pub fn into_raw_intent(
        self,
        mode: &WorthTopologyEditApplicationMode,
    ) -> RawWorthTopologyIntent {
        let mutations = self
            .contracts
            .into_iter()
            .flat_map(|contract| contract.lowered_mutations().to_vec())
            .collect();
        RawWorthTopologyIntent::new(
            mutations,
            WorthTopologyEditContract::mutation_origin_for(mode),
        )
    }
}

#[derive(Debug, Clone)]
pub struct WorthTopologyEditRuntimeTrace {
    pub mode: WorthTopologyEditApplicationMode,
    pub families: Vec<super::types::WorthTopologyEditFamily>,
    pub naming_report: WorthTopologyEditNamingReport,
    pub verified_commit: Option<VerifiedTopologyCommit>,
    pub decision_trace: Option<WorthDecisionTrace>,
    pub integrity_markers: Option<WorthIntegrityMarkers>,
    pub performance_accounting: Option<WorthPerformanceAccounting>,
}

impl WorthTopologyEditRuntimeTrace {
    fn from_batch(batch: &WorthTopologyEditBatch, mode: &WorthTopologyEditApplicationMode) -> Self {
        Self {
            mode: mode.clone(),
            families: batch.families(),
            naming_report: batch.naming_report(),
            verified_commit: None,
            decision_trace: None,
            integrity_markers: None,
            performance_accounting: None,
        }
    }

    fn with_verified_commit(mut self, verified_commit: VerifiedTopologyCommit) -> Self {
        self.verified_commit = Some(verified_commit);
        self
    }

    fn with_authority_envelope(mut self, authority: &WorthTracedTopologyCommit) -> Self {
        self.decision_trace = Some(authority.decision_trace().clone());
        self.integrity_markers = Some(authority.integrity_markers().clone());
        self.performance_accounting = Some(authority.performance_accounting().clone());
        self
    }

    fn with_naming_report(mut self, naming_report: WorthTopologyEditNamingReport) -> Self {
        self.naming_report = naming_report;
        self
    }

    fn with_authority_failure(
        mut self,
        authority: &WorthBoundaryFailure<WorthTopologyAuthorityError>,
    ) -> Self {
        self.decision_trace = Some(authority.decision_trace().clone());
        self.integrity_markers = Some(authority.integrity_markers().clone());
        self.performance_accounting = Some(authority.performance_accounting().clone());
        self
    }
}

#[derive(Debug)]
pub enum WorthTopologyEditError {
    EmptyBatch,
    Authority {
        source: WorthBoundaryFailure<WorthTopologyAuthorityError>,
        trace: WorthTopologyEditRuntimeTrace,
    },
    Read {
        source: WorthTopologyReadError,
        trace: WorthTopologyEditRuntimeTrace,
    },
    Validation {
        source: WorthTopologyValidationError,
        trace: WorthTopologyEditRuntimeTrace,
    },
}

impl std::fmt::Display for WorthTopologyEditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyBatch => write!(f, "topology edit batch must contain at least one contract"),
            Self::Authority { source, trace } => write!(
                f,
                "authority failure for {:?} on {:?}: {source:?}",
                trace.families, trace.mode
            ),
            Self::Read { source, trace } => write!(
                f,
                "read failure for {:?} on {:?}: {source}",
                trace.families, trace.mode
            ),
            Self::Validation { source, trace } => write!(
                f,
                "validation failure for {:?} on {:?}: {source}",
                trace.families, trace.mode
            ),
        }
    }
}

impl std::error::Error for WorthTopologyEditError {}

impl WorthTopologyEditError {
    pub fn trace(&self) -> Option<&WorthTopologyEditRuntimeTrace> {
        match self {
            Self::EmptyBatch => None,
            Self::Authority { trace, .. }
            | Self::Read { trace, .. }
            | Self::Validation { trace, .. } => Some(trace),
        }
    }

    pub fn authority_error(&self) -> Option<&WorthTopologyAuthorityError> {
        match self {
            Self::Authority { source, .. } => Some(source.error()),
            _ => None,
        }
    }

    pub fn read_error(&self) -> Option<&WorthTopologyReadError> {
        match self {
            Self::Read { source, .. } => Some(source),
            _ => None,
        }
    }

    pub fn validation_error(&self) -> Option<&WorthTopologyValidationError> {
        match self {
            Self::Validation { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn authority_error_with_trace(
    source: WorthBoundaryFailure<WorthTopologyAuthorityError>,
    trace: WorthTopologyEditRuntimeTrace,
) -> WorthTopologyEditError {
    let naming_report = match source.error() {
        WorthTopologyAuthorityError::DuplicateCreateKey(key) => {
            trace.naming_report.clone().rejected(format!(
                "duplicate create key `{}` in edit batch",
                key.as_str()
            ))
        }
        WorthTopologyAuthorityError::DuplicateLiveEntityLabel(key) => {
            trace.naming_report.clone().rejected(format!(
                "live topology truth already contains entity label `{}`",
                key.as_str()
            ))
        }
        _ => trace.naming_report.clone(),
    };
    let trace = trace
        .with_authority_failure(&source)
        .with_naming_report(naming_report);
    WorthTopologyEditError::Authority { source, trace }
}

fn read_error_with_trace(
    source: WorthTopologyReadError,
    trace: WorthTopologyEditRuntimeTrace,
) -> WorthTopologyEditError {
    WorthTopologyEditError::Read { source, trace }
}

fn validation_error_with_trace(
    source: WorthTopologyValidationError,
    trace: WorthTopologyEditRuntimeTrace,
) -> WorthTopologyEditError {
    let naming_report = trace
        .naming_report
        .clone()
        .rejected(format!("naming continuity rejected: {}", source.message()));
    let trace = trace.with_naming_report(naming_report);
    WorthTopologyEditError::Validation { source, trace }
}

#[derive(Debug)]
pub struct WorthTopologyEditApplied {
    pub verified_commit: VerifiedTopologyCommit,
    pub derived_validation_report: DerivedTopologyValidationReport,
    pub naming_report: WorthTopologyEditNamingReport,
}

pub type WorthTracedTopologyEditCommit = WorthTracedTopologyCommit;
pub type WorthTracedTopologyEditApplied = WorthBoundaryEnvelope<WorthTopologyEditApplied>;

pub struct WorthTopologyEditRunner<'a> {
    runtime: &'a mut RelationalRuntime,
}

impl<'a> WorthTopologyEditRunner<'a> {
    pub fn new(runtime: &'a mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn apply_traced(
        &mut self,
        batch: WorthTopologyEditBatch,
        mode: WorthTopologyEditApplicationMode,
    ) -> Result<WorthTracedTopologyEditCommit, WorthTopologyEditError> {
        let trace = WorthTopologyEditRuntimeTrace::from_batch(&batch, &mode);
        let intent = batch.into_raw_intent(&mode);
        match mode {
            WorthTopologyEditApplicationMode::Mainline => WorthTopologyAuthority::new(self.runtime)
                .apply_topology_intent_traced(intent)
                .map_err(|error| authority_error_with_trace(error, trace)),
            WorthTopologyEditApplicationMode::BranchLocal(branch_id) => {
                WorthTopologyAuthority::new(self.runtime)
                    .apply_topology_intent_on_branch_traced(intent, branch_id)
                    .map_err(|error| authority_error_with_trace(error, trace))
            }
        }
    }

    pub fn apply_and_inspect_traced(
        &mut self,
        batch: WorthTopologyEditBatch,
        mode: WorthTopologyEditApplicationMode,
    ) -> Result<WorthTracedTopologyEditApplied, WorthTopologyEditError> {
        let trace = WorthTopologyEditRuntimeTrace::from_batch(&batch, &mode);
        let naming_report = trace.naming_report.clone();
        let authority = match mode.clone() {
            WorthTopologyEditApplicationMode::Mainline => WorthTopologyAuthority::new(self.runtime)
                .apply_topology_intent_traced(batch.into_raw_intent(&mode)),
            WorthTopologyEditApplicationMode::BranchLocal(branch_id) => {
                WorthTopologyAuthority::new(self.runtime)
                    .apply_topology_intent_on_branch_traced(batch.into_raw_intent(&mode), branch_id)
            }
        }
        .map_err(|error| authority_error_with_trace(error, trace.clone()))?;
        let verified_commit = authority.primary_result().clone();
        let trace = trace
            .with_verified_commit(verified_commit.clone())
            .with_authority_envelope(&authority);
        let reader = WorthTopologyReader::new(self.runtime);
        let basis = reader.read_basis_from_verified_commit(&verified_commit);
        let read_view = reader
            .read_view(&basis)
            .map_err(|error| read_error_with_trace(error, trace.clone()))?;
        validate_named_topology_truth(&read_view)
            .map_err(|error| validation_error_with_trace(error, trace.clone()))?;
        let staged = reader
            .stage(&basis)
            .map_err(|error| read_error_with_trace(error, trace.clone()))?;

        Ok(WorthBoundaryEnvelope::success(
            WorthTopologyEditApplied {
                verified_commit,
                derived_validation_report: staged.validation().clone(),
                naming_report,
            },
            authority.warnings().to_vec(),
            authority.decision_trace().clone(),
            authority.integrity_markers().clone(),
            authority.performance_accounting().clone(),
        ))
    }
}
